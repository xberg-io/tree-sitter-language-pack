//! Declaration coverage for the code-shaped repository formats: SQL DDL,
//! Justfiles, Dockerfiles, C sources and headers, and Cedar policies. Their
//! grammars name nodes the generic matcher misreads or never sees, so each
//! has its own adapter, exercised here on the shapes real PostgreSQL
//! bootstrap scripts, Justfiles, multi-stage Dockerfiles, C headers and
//! policy sets take.
#![allow(clippy::unwrap_used, clippy::expect_used)] // ~keep: a failed setup step in a test must abort loudly
#![allow(clippy::print_stdout)] // ~keep: the deep-declarator probe reports to its parent process on stdout

use tree_sitter_language_pack::{ProcessConfig, ProcessResult, StructureItem, StructureKind, process};

fn extract(source: &str, language: &str) -> ProcessResult {
    process(source, &ProcessConfig::new(language).all())
        .expect("real grammar required; build with TSLP_LANGUAGES=sql,just,dockerfile,c,cedar")
}

/// (name, kind, start_line, end_line) for each item, top level only.
fn summary(items: &[StructureItem]) -> Vec<(Option<&str>, &StructureKind, usize, usize)> {
    items
        .iter()
        .map(|item| {
            (
                item.name.as_deref(),
                &item.kind,
                item.span.start_line,
                item.span.end_line,
            )
        })
        .collect()
}

fn other(label: &str) -> StructureKind {
    StructureKind::Other(label.to_string())
}

const SQL_DDL: &str = "\
-- comment
CREATE SCHEMA IF NOT EXISTS engram;
CREATE EXTENSION IF NOT EXISTS vector;
CREATE ROLE engram_app LOGIN;
CREATE TABLE engram.thoughts (
  id uuid PRIMARY KEY,
  tenant_id uuid NOT NULL
);
CREATE TABLE IF NOT EXISTS \"Quoted\".\"My Table\" (id int);
CREATE UNIQUE INDEX thoughts_tenant_idx ON engram.thoughts (tenant_id, id);
CREATE INDEX CONCURRENTLY IF NOT EXISTS other_idx ON thoughts USING btree (id);
CREATE OR REPLACE FUNCTION engram_current_tenant()
RETURNS uuid
LANGUAGE sql STABLE AS $$
  SELECT current_setting('engram.tenant_id', true)::uuid
$$;
CREATE FUNCTION public.sync_lexemes() RETURNS trigger LANGUAGE plpgsql AS $body$
BEGIN
  INSERT INTO x VALUES (1);
  RETURN NEW;
END;
$body$;
CREATE TRIGGER engram_sync_symbol_lexemes AFTER INSERT OR UPDATE ON symbol_cards
  FOR EACH ROW EXECUTE FUNCTION sync_lexemes();
CREATE POLICY tenant_isolation ON engram.thoughts
  USING (tenant_id = engram_current_tenant());
CREATE OR REPLACE VIEW engram.recent AS SELECT * FROM engram.thoughts;
CREATE MATERIALIZED VIEW engram.stats AS SELECT count(*) FROM engram.thoughts;
CREATE TYPE mood AS ENUM ('sad', 'ok');
CREATE TEMP TABLE scratch (id int);
CREATE SEQUENCE seq1;
ALTER TABLE engram.thoughts ENABLE ROW LEVEL SECURITY;
SELECT 1;
";

#[test]
fn sql_ddl_objects_have_kinds_qualified_names_and_statement_spans() {
    let result = extract(SQL_DDL, "sql");
    assert_eq!(
        summary(&result.structure),
        vec![
            (Some("engram"), &StructureKind::Module, 1, 1),
            (Some("engram_app"), &other("Role"), 3, 3),
            (Some("engram.thoughts"), &StructureKind::Struct, 4, 7),
            (Some("Quoted.My Table"), &StructureKind::Struct, 8, 8),
            (Some("thoughts_tenant_idx"), &other("Index"), 9, 9),
            (Some("other_idx"), &other("Index"), 10, 10),
            (Some("engram_current_tenant"), &StructureKind::Function, 11, 15),
            (Some("public.sync_lexemes"), &StructureKind::Function, 16, 21),
            (
                Some("symbol_cards.engram_sync_symbol_lexemes"),
                &other("Trigger"),
                22,
                23
            ),
            (Some("engram.thoughts.tenant_isolation"), &other("Policy"), 24, 25),
            (Some("engram.recent"), &StructureKind::Struct, 26, 26),
            (Some("engram.stats"), &StructureKind::Struct, 27, 27),
            (Some("mood"), &other("Type"), 28, 28),
            (Some("scratch"), &StructureKind::Struct, 29, 29),
        ]
    );
    // The statement span includes its terminator and nothing after it.
    let function = &result.structure[6].span;
    assert!(SQL_DDL[function.start_byte..function.end_byte].ends_with("$$;"));
    assert!(result.structure.iter().all(|item| item.children.is_empty()));
    assert!(result.symbols.is_empty(), "SQL emits no flat symbols");
}

#[test]
fn sql_declarations_folded_into_error_recovery_keep_exact_spans() {
    // `SET search_path` in a function header and the GRANT are not in the
    // grammar; recovery folds everything after each into an ERROR node.
    let source = "\
CREATE OR REPLACE FUNCTION engram_purge(p_tenant text)
RETURNS jsonb
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public, pg_catalog
AS $fn$
DECLARE
  n_cards integer;
BEGIN
  DELETE FROM symbol_cards WHERE tenant_id = p_tenant;
  RETURN jsonb_build_object('cards', n_cards);
END;
$fn$;
GRANT SELECT ON symbol_cards TO engram_app;
CREATE UNIQUE INDEX symbol_cards_id_key ON symbol_cards (id);
CREATE POLICY symbol_cards_rls ON symbol_cards
  USING (tenant_id = engram_current_tenant());
CREATE OR REPLACE FUNCTION engram_sync() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
  RETURN NEW;
END;
$$;
CREATE TABLE code_repos (id uuid PRIMARY KEY);
";
    let result = extract(source, "sql");
    assert!(result.metrics.error_count > 0, "the fixture must exercise recovery");
    assert_eq!(
        summary(&result.structure),
        vec![
            (Some("engram_purge"), &StructureKind::Function, 0, 12),
            (Some("symbol_cards_id_key"), &other("Index"), 14, 14),
            (Some("symbol_cards.symbol_cards_rls"), &other("Policy"), 15, 16),
            (Some("engram_sync"), &StructureKind::Function, 17, 21),
            (Some("code_repos"), &StructureKind::Struct, 22, 22),
        ]
    );
    for item in &result.structure {
        let text = &source[item.span.start_byte..item.span.end_byte];
        assert!(text.starts_with("CREATE"), "{text}");
        assert!(text.ends_with(';'), "{text}");
    }
}

#[test]
fn sql_lost_names_variables_and_bodies_are_not_declarations() {
    // A CREATE inside a recognised body belongs to the routine. A psql
    // variable, a keyword where the name should be (GRANT CREATE ON), and a
    // format placeholder are not names, so those statements yield nothing.
    let source = "\
CREATE OR REPLACE FUNCTION counted() RETURNS integer LANGUAGE plpgsql AS $$
DECLARE
  v_total integer;
BEGIN
  CREATE TEMP TABLE inner_scratch (id int);
  SELECT count(*) INTO v_total FROM symbol_cards;
  RETURN v_total;
END;
$$;
GRANT CREATE ON SCHEMA public TO engram;
CREATE DATABASE :db_name OWNER engram;
CREATE ROLE :\"role_name\" LOGIN;
CREATE FUNCTION %s(%s) RETURNS %s;
";
    let result = extract(source, "sql");
    assert!(result.metrics.error_count > 0, "the fixture must exercise recovery");
    assert_eq!(
        summary(&result.structure),
        vec![(Some("counted"), &StructureKind::Function, 0, 8)]
    );
}

#[test]
fn sql_recovery_reads_a_schema_the_lexer_handed_over_as_a_keyword() {
    // Shrunk from a real proof file: the first body derails the parser, which
    // runs the function past its own closing quote and hands the next header
    // over as `keyword_public`, `.`, then the bare name.
    let source = "\
    CREATE OR REPLACE FUNCTION public.engram_floor_complete(p_tenant text, p_set_id uuid)
     RETURNS boolean
    AS $function$
    DECLARE
      -- SQLSTATE 55P03 here instead of parking this tenant connection.
                 AND  sv.card_id   = sc.id
                 AND  sv.set_id    = sc.set_id
             );

      RETURN v_floorless = 0;
    END;
    $function$
;
    CREATE OR REPLACE FUNCTION public.engram_complete(p_tenant text, p_floor_complete boolean)
     RETURNS text
     LANGUAGE plpgsql
     SECURITY DEFINER
     SET search_path TO 'public', 'pg_catalog'
    AS $function$
    DECLARE
      v_repo             text;
    BEGIN
      RETURN 1;
    END;
    $function$
;
";
    let result = extract(source, "sql");
    assert_eq!(
        summary(&result.structure),
        vec![
            (Some("public.engram_floor_complete"), &StructureKind::Function, 0, 12),
            (Some("public.engram_complete"), &StructureKind::Function, 13, 25),
        ]
    );
    for item in &result.structure {
        let text = &source[item.span.start_byte..item.span.end_byte];
        assert!(text.starts_with("CREATE") && text.ends_with("$function$\n;"), "{text}");
    }
}

#[test]
fn sql_table_the_parser_ran_past_its_terminator_ends_there() {
    // Shrunk from a real DDL proof: the REFERENCES clause and the GRANTs after
    // it fold the next CREATE TABLE into the first table's node, and the lexer
    // drops that second table's name token, so it yields nothing rather than
    // a name it never had.
    let source = "\
CREATE TABLE tinker_platform_release_files (
  platform_file_id     uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
  platform_release_id  uuid        NOT NULL
    REFERENCES tinker_platform_releases (platform_release_id) ON DELETE CASCADE,
  CONSTRAINT tinker_platform_release_files_path_key UNIQUE (platform_release_id, rel_path)
);
GRANT SELECT, INSERT, UPDATE, DELETE ON tinker_platform_release_files TO kyroco_publisher;
REVOKE INSERT, UPDATE, DELETE ON tinker_platform_release_files FROM engram_app;
CREATE TABLE tinker_promotion_audit (
  audit_id        uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
  CONSTRAINT tinker_promotion_audit_direction_chk
    CHECK (direction IN ('promote','demote'))
);
";
    let result = extract(source, "sql");
    assert_eq!(
        summary(&result.structure),
        vec![(Some("tinker_platform_release_files"), &StructureKind::Struct, 0, 5)]
    );
    let table = &result.structure[0].span;
    assert!(source[table.start_byte..table.end_byte].ends_with(");"));
}

const JUSTFILE: &str = "\
set shell := [\"bash\", \"-eu\", \"-c\"]

mod kyroco 'justfiles/kyroco.just'
import 'other.just'

MIX := \"/opt/homebrew/bin/mix\"

# Run the proof.
[private]
default:
    @just --list

pre-pr *args:
    cd {{DIR}} && ./scripts/pre-pr.sh {{args}}

alias pp := pre-pr

[group(\"ci\")]
@build target=\"debug\" flag: deps compile
    echo {{target}}
    echo done

_hidden:
    echo hidden

deps:
    mix deps.get
compile: deps
    mix compile
";

#[test]
fn justfile_recipes_aliases_and_modules_are_declarations_with_trimmed_spans() {
    let result = extract(JUSTFILE, "just");
    assert_eq!(result.metrics.error_count, 0);
    assert_eq!(
        summary(&result.structure),
        vec![
            (Some("kyroco"), &StructureKind::Module, 2, 2),
            (Some("default"), &StructureKind::Function, 8, 10),
            (Some("pre-pr"), &StructureKind::Function, 12, 13),
            (Some("pp"), &other("Alias"), 15, 15),
            (Some("build"), &StructureKind::Function, 17, 20),
            (Some("_hidden"), &StructureKind::Function, 22, 23),
            (Some("deps"), &StructureKind::Function, 25, 26),
            (Some("compile"), &StructureKind::Function, 27, 28),
        ]
    );
    let build = &result.structure[4].span;
    assert!(JUSTFILE[build.start_byte..build.end_byte].starts_with("[group("));
    assert!(JUSTFILE[build.start_byte..build.end_byte].ends_with("echo done"));
    assert!(result.symbols.is_empty(), "Just emits no flat symbols");
}

const DOCKERFILE: &str = "\
# syntax=docker/dockerfile:1
ARG ERLANG_VERSION=29.0.3
ARG DEBIAN_VERSION=bookworm
FROM hexpm/erlang:${ERLANG_VERSION}-debian-${DEBIAN_VERSION} AS toolchain_base
ARG ELIXIR_VERSION
ENV MIX_ENV=${MIX_ENV} \\
    LANG=C.UTF-8
RUN apt-get update \\
  && apt-get install -y git
COPY . /app

# The next stage.
FROM toolchain_base AS deps_base
WORKDIR /app
ENV RUSTUP_HOME=/opt/rustup
FROM debian:${DEBIAN_VERSION}-slim as runtime_api
ARG MIX_ENV
ENV LANG C.UTF-8
FROM nginx:1.27-alpine
ARG MIX_ENV
CMD [\"nginx\"]
";

#[test]
fn dockerfile_named_stages_nest_their_args_and_env_pairs() {
    let result = extract(DOCKERFILE, "dockerfile");
    assert_eq!(result.metrics.error_count, 0);
    assert_eq!(
        summary(&result.structure),
        vec![
            (Some("ERLANG_VERSION"), &other("Constant"), 1, 1),
            (Some("DEBIAN_VERSION"), &other("Constant"), 2, 2),
            (Some("toolchain_base"), &StructureKind::Module, 3, 9),
            (Some("deps_base"), &StructureKind::Module, 12, 14),
            (Some("runtime_api"), &StructureKind::Module, 15, 17),
            (Some("MIX_ENV"), &other("Constant"), 19, 19),
        ]
    );
    assert_eq!(
        summary(&result.structure[2].children),
        vec![
            (Some("ELIXIR_VERSION"), &other("Constant"), 4, 4),
            (Some("MIX_ENV"), &other("Constant"), 5, 6),
            (Some("LANG"), &other("Constant"), 5, 6),
        ]
    );
    assert_eq!(
        summary(&result.structure[3].children),
        vec![(Some("RUSTUP_HOME"), &other("Constant"), 14, 14)]
    );
    assert_eq!(
        summary(&result.structure[4].children),
        vec![
            (Some("MIX_ENV"), &other("Constant"), 16, 16),
            (Some("LANG"), &other("Constant"), 17, 17),
        ]
    );
    let stage = &result.structure[2].span;
    assert!(DOCKERFILE[stage.start_byte..stage.end_byte].ends_with("COPY . /app"));
    assert!(result.symbols.is_empty(), "Dockerfile emits no flat symbols");
}

const C_SOURCE: &str = "\
#include <stdio.h>
#define LIMIT 42
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define FLAG

struct point {
  int x;
  int y;
};

typedef struct point point_t;

typedef struct {
  int w;
} bare_t;

typedef struct node {
  struct node *next;
} node_t;

union number {
  int i;
  float f;
};

enum color { RED, GREEN };

typedef enum { LOW, HIGH } level_t;

typedef int (*callback_t)(int);

static int counter = 0;
const int limit = 3;
extern int external;
struct point origin = {0, 0};
int (*fnptr)(int);

static inline int helper(int v) {
  struct local { int z; };
  int inner = v;
  return inner;
}

int prototype(int a);
char *pointer_prototype(const char *s);

int main(int argc, char **argv) {
  return MAX(argc, LIMIT);
}
";

#[test]
fn c_definitions_prototypes_aggregates_typedefs_and_macros_are_declarations() {
    let result = extract(C_SOURCE, "c");
    assert_eq!(result.metrics.error_count, 0);
    assert_eq!(
        summary(&result.structure),
        vec![
            (Some("LIMIT"), &other("Constant"), 1, 1),
            (Some("MAX"), &other("Macro"), 2, 2),
            (Some("FLAG"), &other("Constant"), 3, 3),
            (Some("point"), &StructureKind::Struct, 5, 8),
            (Some("point_t"), &other("Type"), 10, 10),
            (Some("bare_t"), &other("Type"), 12, 14),
            (Some("node_t"), &other("Type"), 16, 18),
            (Some("node"), &StructureKind::Struct, 16, 18),
            (Some("number"), &other("Union"), 20, 23),
            (Some("color"), &StructureKind::Enum, 25, 25),
            (Some("level_t"), &other("Type"), 27, 27),
            (Some("callback_t"), &other("Type"), 29, 29),
            (Some("helper"), &StructureKind::Function, 37, 41),
            (Some("prototype"), &StructureKind::Function, 43, 43),
            (Some("pointer_prototype"), &StructureKind::Function, 44, 44),
            (Some("main"), &StructureKind::Function, 46, 48),
        ]
    );
    // ~keep A macro's span stops at its text, not at the newline the grammar swallows.
    let limit = &result.structure[0].span;
    assert_eq!(&C_SOURCE[limit.start_byte..limit.end_byte], "#define LIMIT 42");
    // ~keep A top-level aggregate runs through its own terminator.
    let point = &result.structure[3].span;
    assert!(C_SOURCE[point.start_byte..point.end_byte].ends_with("};"));
    assert!(result.structure.iter().all(|item| item.children.is_empty()));
    assert!(result.symbols.is_empty(), "C emits no flat symbols");
}

/// The line and column of every span, recomputed from the source text alone.
///
/// Independent of how the adapters arrive at a position, so it holds whether
/// they count from the top of the file or from a position the parser gave
/// them.
fn line_and_column(source: &str, byte: usize) -> (usize, usize) {
    let before = &source.as_bytes()[..byte.min(source.len())];
    let line = before.iter().filter(|b| **b == b'\n').count();
    let column = before.len() - before.iter().rposition(|b| *b == b'\n').map_or(0, |at| at + 1);
    (line, column)
}

fn assert_spans_agree_with_the_source(source: &str, language: &str) {
    let result = extract(source, language);
    assert!(
        !result.structure.is_empty(),
        "{language}: nothing extracted, so the spans prove nothing"
    );
    let mut pending: Vec<&StructureItem> = result.structure.iter().collect();
    while let Some(item) = pending.pop() {
        pending.extend(item.children.iter());
        let span = &item.span;
        assert!(
            span.start_byte <= span.end_byte && span.end_byte <= source.len(),
            "{language}: {:?} has byte range {}..{} in a {} byte source",
            item.name,
            span.start_byte,
            span.end_byte,
            source.len()
        );
        assert!(
            source.is_char_boundary(span.start_byte) && source.is_char_boundary(span.end_byte),
            "{language}: {:?} splits a character",
            item.name
        );
        assert_eq!(
            (span.start_line, span.start_column),
            line_and_column(source, span.start_byte),
            "{language}: {:?} start position disagrees with its byte offset",
            item.name
        );
        assert_eq!(
            (span.end_line, span.end_column),
            line_and_column(source, span.end_byte),
            "{language}: {:?} end position disagrees with its byte offset",
            item.name
        );
    }
}

/// Positions are anchored on the position the parser already recorded for a
/// node, rather than counted from the top of the file for every declaration,
/// which cost O(declarations * length) to extract. The saving is only worth
/// having if the positions are unchanged, so every span each adapter reports
/// is checked against the source it came from.
///
/// Columns are counted in bytes, so a multi-byte character is where an
/// anchored count and a counted-from-zero one would part company; the
/// fixtures carry one on a line before a declaration and inside one.
#[test]
fn every_declaration_span_agrees_with_its_own_source() {
    assert_spans_agree_with_the_source(C_SOURCE, "c");
    assert_spans_agree_with_the_source(C_HEADER, "c");
    assert_spans_agree_with_the_source(SQL_DDL, "sql");
    assert_spans_agree_with_the_source(JUSTFILE, "just");
    assert_spans_agree_with_the_source(DOCKERFILE, "dockerfile");
    assert_spans_agree_with_the_source(CEDAR, "cedar");

    assert_spans_agree_with_the_source(
        "\
// primera línea, uno más
int café(int x);

/* ± ² ³ */
struct punto { int x; };

#define TAMAÑO 10
int main(void) { return TAMAÑO; }
",
        "c",
    );
    assert_spans_agree_with_the_source(
        "-- ¿cuántos?\nCREATE TABLE café (id int);\nCREATE VIEW ± AS SELECT 1;\n",
        "sql",
    );
}

/// C spells a function declaration more than one way, and requiring the
/// function declarator to apply to a bare identifier saw only the plainest.
/// A redundant pair of parentheses around the name is legal, and a function
/// returning a function pointer puts its name under an inner declarator.
/// Neither is a variable, and the function pointer variable that shares their
/// shape still is.
#[test]
fn c_prototypes_behind_parentheses_and_returned_function_pointers_are_functions() {
    let source = "\
int (parenthesised)(int);
int (*factory(void))(int);
char *(*table_lookup(const char *key))(int, int);
int (*variable)(int);
int (*table[4])(int);
extern int external;
";
    let result = extract(source, "c");
    assert_eq!(
        summary(&result.structure),
        vec![
            (Some("parenthesised"), &StructureKind::Function, 0, 0),
            (Some("factory"), &StructureKind::Function, 1, 1),
            (Some("table_lookup"), &StructureKind::Function, 2, 2),
        ],
        "a redundant parenthesis and a returned function pointer declare functions; \
         a function pointer variable, an array of them and a scalar declare none"
    );
}

const C_HEADER: &str = "\
// A guarded header.
#ifndef WIDGET_H
#define WIDGET_H

#ifdef __cplusplus
extern \"C\" {
#endif

#if defined(WIDGET_SMALL)
int widget_small(void);
#else
int widget_large(void);
#endif

void widget_free(char *pointer);

#ifdef __cplusplus
}
#endif

#endif
";

#[test]
fn c_header_guards_conditionals_and_linkage_blocks_are_transparent() {
    let result = extract(C_HEADER, "c");
    // ~keep The grammar pairs the `extern "C" {` brace across the `#endif` that
    // ~keep closes its guard and reports one error node for it; the prototypes
    // ~keep inside are parsed all the same, which is what this proves.
    assert!(result.metrics.error_count <= 1);
    assert_eq!(
        summary(&result.structure),
        vec![
            (Some("WIDGET_H"), &other("Constant"), 2, 2),
            (Some("widget_small"), &StructureKind::Function, 9, 9),
            (Some("widget_large"), &StructureKind::Function, 11, 11),
            (Some("widget_free"), &StructureKind::Function, 14, 14),
        ]
    );
}

/// Set by the parent test so the child process runs the deep-declarator probe for real.
const C_DEEP_DECLARATOR_PROBE_ENV: &str = "TSLP_C_DEEP_DECLARATOR_PROBE";
const C_DEEP_DECLARATOR_STARS: usize = 100_000;
const C_DEEP_DECLARATOR_SENTINEL: &str = "c deep declarator probe completed";

/// The probe itself. Ignored so a plain run never executes it in-process: a stack
/// overflow is an abort, not a panic, and would take every other test in this
/// binary down with it. The parent below re-executes this binary with the env var
/// set and asks for exactly this test.
#[test]
#[ignore]
fn c_deep_declarator_probe() {
    if std::env::var_os(C_DEEP_DECLARATOR_PROBE_ENV).is_none() {
        return;
    }
    let stars = "*".repeat(C_DEEP_DECLARATOR_STARS);
    // ~keep One prototype, one variable and one typedef, each behind the same
    // ~keep pointer chain: the prototype and the typedef walk the chain to their
    // ~keep name, the variable walks it to learn it declares nothing.
    let source = format!("int {stars}deep(void);\nint {stars}name;\ntypedef int {stars}alias;\n");
    // ~keep 2 MiB is the smallest stack the library runs on (tokio spawn_blocking and the
    // ~keep Node, Python and JVM worker threads); the default test thread is not that small.
    let worker = std::thread::Builder::new()
        .stack_size(2 * 1024 * 1024)
        .spawn(move || {
            let result = extract(&source, "c");
            (result.metrics.error_count, result.structure)
        })
        .expect("spawn probe thread");
    let (errors, items) = worker.join().expect("probe thread must not panic");
    assert_eq!(errors, 0, "the fixture must parse cleanly for the chain to be real");
    assert_eq!(
        summary(&items),
        vec![
            (Some("deep"), &StructureKind::Function, 0, 0),
            (Some("alias"), &other("Type"), 2, 2),
        ]
    );
    println!("{C_DEEP_DECLARATOR_SENTINEL}");
}

/// A declarator wrapped in 100,000 pointers must not abort the process.
///
/// The C grammar nests one `pointer_declarator` per `*`, so a name finder that
/// recurses per level walks off a 2 MiB stack long before the depth guard the
/// item walk shares can see it. An abort cannot be caught in-process, so the
/// probe runs in a child and this test reads its exit status and its sentinel line;
/// the sentinel is what stops a filtered-out or silently skipped probe from passing
/// as a survival.
#[test]
fn c_deep_declarator_survives_a_two_mebibyte_stack() {
    let exe = std::env::current_exe().expect("test binary path");
    let output = std::process::Command::new(exe)
        .args(["--ignored", "--exact", "c_deep_declarator_probe", "--nocapture"])
        .env(C_DEEP_DECLARATOR_PROBE_ENV, "1")
        .output()
        .expect("spawn the probe child process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() && stdout.contains(C_DEEP_DECLARATOR_SENTINEL),
        "the deep declarator probe did not survive (status {:?})\nstdout:\n{stdout}\nstderr:\n{stderr}",
        output.status
    );
}

const CEDAR: &str = "\
// leading comment with @id(\"not_a_policy\") in it
@id(\"allow_read\")
@note(\"first\")
permit(principal, action == Action::\"read\", resource);

@id(\"deny_delete\")
forbid(
  principal,
  action == Action::\"delete\",
  resource
)
when { principal.role == \"guest\" };

permit(principal, action, resource);

@id(\"templated\")
permit(principal == ?principal, action, resource in ?resource);

@id(\"allow_read\")
permit(principal, action == Action::\"list\", resource) unless { resource.locked };
";

#[test]
fn cedar_policies_are_named_by_their_id_annotation() {
    let result = extract(CEDAR, "cedar");
    assert_eq!(result.metrics.error_count, 0);
    assert_eq!(
        summary(&result.structure),
        vec![
            (Some("allow_read"), &other("Policy"), 1, 3),
            (Some("deny_delete"), &other("Policy"), 5, 11),
            (Some("templated"), &other("Policy"), 15, 16),
            (Some("allow_read"), &other("Policy"), 18, 19),
        ]
    );
    let first = &result.structure[0].span;
    assert!(CEDAR[first.start_byte..first.end_byte].starts_with("@id(\"allow_read\")"));
    assert!(CEDAR[first.start_byte..first.end_byte].ends_with("resource);"));
    assert!(result.symbols.is_empty(), "Cedar emits no flat symbols");
}
