//! Deterministic, seeded source generators shared by the splitter/chunking tests and benches. Std-only so it can be `#[path]`-included from
//! in-crate tests, integration tests and benches alike.
//!
//! Every generator grows its output past a byte target, so one knob scales a shape
//! 1x/2x/4x/8x without changing its structure. Nothing here is written to disk.

use std::fmt::Write as _;

/// Small xorshift64* generator; fixed seeds make every generated input reproducible.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform in `0..n` (`n > 0`).
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    pub fn range(&mut self, lo: usize, hi: usize) -> usize {
        lo + self.below(hi - lo + 1)
    }
}

/// A named generator for one language.
pub struct Shape {
    pub name: &'static str,
    pub lang: &'static str,
    pub generate: fn(&mut Rng, usize) -> String,
}

fn fill(target: usize, mut step: impl FnMut(&mut String, usize)) -> String {
    let mut out = String::with_capacity(target + 256);
    let mut i = 0;
    while out.len() < target {
        step(&mut out, i);
        i += 1;
    }
    out
}

fn words(rng: &mut Rng, n: usize) -> String {
    const W: &[&str] = &[
        "alpha", "beta", "gamma", "delta", "value", "item", "node", "count", "index", "total",
    ];
    (0..n).map(|_| W[rng.below(W.len())]).collect::<Vec<_>>().join("_")
}

fn words_between(rng: &mut Rng, lo: usize, hi: usize) -> String {
    let n = rng.range(lo, hi);
    words(rng, n)
}

fn rust_fns(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let body = words_between(rng, 1, 6);
        let _ = writeln!(o, "fn f{i:07}(a: i32) -> i32 {{ let {body} = a; g({body}) }}");
    })
}

fn python_defs(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let body = words_between(rng, 1, 6);
        let _ = write!(o, "def f{i:07}(a):\n    {body} = a\n    return {body}\n\n");
    })
}

fn js_fns(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let body = words_between(rng, 1, 6);
        let _ = writeln!(o, "function f{i:07}(a) {{ const {body} = a; return {body}; }}");
    })
}

fn c_fns(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let body = words_between(rng, 1, 6);
        let _ = writeln!(o, "int f{i:07}(int a) {{ int {body} = a; return {body}; }}");
    })
}

fn go_fns(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("package main\n\n");
    s.push_str(&fill(target, |o, i| {
        let body = words_between(rng, 1, 6);
        let _ = writeln!(o, "func f{i:07}(a int) int {{ {body} := a; return {body} }}");
    }));
    s
}

fn java_methods(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("class Big {\n");
    s.push_str(&fill(target, |o, i| {
        let body = words_between(rng, 1, 6);
        let _ = writeln!(o, "  int f{i:07}(int a) {{ int {body} = a; return {body}; }}");
    }));
    s.push_str("}\n");
    s
}

fn rust_nested(rng: &mut Rng, target: usize) -> String {
    let mut s = String::new();
    let mut m = 0;
    while s.len() < target {
        let _ = writeln!(
            s,
            "pub mod m{m:05} {{\n    pub struct S{m:05} {{ pub x: i32 }}\n    impl S{m:05} {{"
        );
        for k in 0..rng.range(2, 8) {
            let body = words_between(rng, 1, 5);
            let _ = writeln!(
                s,
                "        /// doc {k}\n        pub fn method{k}(&self) -> i32 {{ let {body} = self.x; {body} }}"
            );
        }
        s.push_str("    }\n}\n");
        m += 1;
    }
    s
}

fn json_array(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("[");
    let mut first = true;
    while s.len() < target {
        if !first {
            s.push_str(",\n");
        }
        first = false;
        let _ = write!(
            s,
            "{{\"id\": {}, \"name\": \"{}\", \"v\": [1, 2, 3]}}",
            rng.below(1_000_000),
            words(rng, 2)
        );
    }
    s.push_str("]\n");
    s
}

fn json_object(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("{");
    let mut i = 0;
    while s.len() < target {
        if i > 0 {
            s.push_str(",\n");
        }
        let _ = write!(
            s,
            "\"key{i:07}\": {{\"a\": {}, \"b\": \"{}\"}}",
            rng.below(1000),
            words(rng, 3)
        );
        i += 1;
    }
    s.push_str("}\n");
    s
}

fn json_one_line(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("[");
    while s.len() < target {
        let _ = write!(s, "{},", rng.below(1_000_000));
    }
    s.push_str("0]");
    s
}

fn json_numbers_newline_separated(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("[\n");
    while s.len() < target {
        let _ = writeln!(s, "  {},", rng.below(1_000_000));
    }
    s.push_str("  0\n]\n");
    s
}

fn nested_brackets(_rng: &mut Rng, target: usize) -> String {
    // Depth tracks the target but stays below the traversal cap so the splitter's
    // own depth handling, not the truncation fallback, is what is measured.
    let depth = (target / 1024).clamp(8, 400);
    format!("{}1{}\n", "[".repeat(depth), "]".repeat(depth))
}

fn nested_objects(_rng: &mut Rng, target: usize) -> String {
    let depth = (target / 1024).clamp(8, 200);
    let mut s = String::new();
    for _ in 0..depth {
        s.push_str("{\"k\": ");
    }
    s.push('1');
    for _ in 0..depth {
        s.push('}');
    }
    s.push('\n');
    s
}

fn yaml_map(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let _ = write!(
            o,
            "item{i:07}:\n  name: {}\n  count: {}\n  tags: [a, b]\n",
            words(rng, 2),
            rng.below(1000)
        );
    })
}

fn yaml_list(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, _| {
        let _ = write!(o, "- name: {}\n  count: {}\n", words(rng, 2), rng.below(1000));
    })
}

fn csv_rows(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("id,name,value,note\n");
    s.push_str(&fill(target, |o, i| {
        let _ = writeln!(o, "{i},{},{},\"{}\"", words(rng, 1), rng.below(100_000), words(rng, 3));
    }));
    s
}

fn markdown_sections(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let _ = write!(
            o,
            "## Section {i}\n\n{}. {}.\n\n- {}\n- {}\n\n```rust\nfn x{i}() {{}}\n```\n\n",
            words(rng, 6),
            words(rng, 8),
            words(rng, 3),
            words(rng, 3)
        );
    })
}

fn toml_tables(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let _ = write!(
            o,
            "[section{i:07}]\nname = \"{}\"\ncount = {}\nlist = [1, 2, 3]\n\n",
            words(rng, 2),
            rng.below(1000)
        );
    })
}

fn js_one_huge_line(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("var a = [");
    while s.len() < target {
        let _ = write!(s, "{},", rng.below(1_000_000));
    }
    s.push_str("0];\n");
    s
}

fn rust_huge_string_line(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("const S: &str = \"");
    while s.len() < target {
        s.push_str(&words(rng, 3));
        s.push(' ');
    }
    s.push_str("\";\n");
    s
}

fn rust_cjk_emoji(rng: &mut Rng, target: usize) -> String {
    const IDENTS: &[&str] = &["数据", "处理", "ファイル", "변수", "значение"];
    const GLYPHS: &[&str] = &["日本語", "漢字🙂", "😀😀", "한국어", "👨‍👩‍👧"];
    fill(target, |o, i| {
        let id = IDENTS[rng.below(IDENTS.len())];
        let g = GLYPHS[rng.below(GLYPHS.len())];
        let _ = writeln!(
            o,
            "// {g} コメント 😀\nfn {id}{i:06}(a: i32) -> &'static str {{ let _ = a; \"{g}{g}{g}\" }}"
        );
    })
}

fn python_cjk_emoji(rng: &mut Rng, target: usize) -> String {
    const GLYPHS: &[&str] = &["日本語", "漢字🙂", "😀😀", "한국어", "👨‍👩‍👧"];
    fill(target, |o, i| {
        let g = GLYPHS[rng.below(GLYPHS.len())];
        let _ = write!(
            o,
            "def 函数{i:06}(a):\n    \"\"\"{g} docstring 😀\"\"\"\n    return \"{g}{g}\"\n\n"
        );
    })
}

fn rust_crlf(rng: &mut Rng, target: usize) -> String {
    rust_fns(rng, target).replace('\n', "\r\n")
}

fn python_crlf(rng: &mut Rng, target: usize) -> String {
    python_defs(rng, target).replace('\n', "\r\n")
}

fn python_imports(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let _ = writeln!(o, "from pkg{}.mod{i:06} import name{i:06}, other{i:06}", rng.below(50));
    })
}

fn python_comments_docstrings(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let _ = write!(
            o,
            "# comment {i} {}\nclass C{i:06}:\n    \"\"\"Doc {}.\"\"\"\n\n    def m(self):\n        # inner {i}\n        return {i}\n\n",
            words(rng, 3),
            words(rng, 4)
        );
    })
}

fn js_exports(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let _ = writeln!(
            o,
            "import {{ a{i:06} }} from './m{i:06}.js';\nexport const v{i:06} = a{i:06} + {};",
            rng.below(100)
        );
    })
}

fn python_broken(rng: &mut Rng, target: usize) -> String {
    fill(target, |o, i| {
        let _ = write!(o, "def f{i:06}(:\n    pass\nclass ???{}:\n", rng.below(10));
    })
}

fn python_giant_class(rng: &mut Rng, target: usize) -> String {
    let mut s = String::from("class Big:\n");
    s.push_str(&fill(target, |o, i| {
        let body = words_between(rng, 1, 6);
        let _ = write!(
            o,
            "    def m{i:07}(self):\n        {body} = {i}\n        return {body}\n\n"
        );
    }));
    s
}

/// Every generated shape, one per structurally distinct scaling hazard.
pub const SHAPES: &[Shape] = &[
    Shape {
        name: "rust_many_fns",
        lang: "rust",
        generate: rust_fns,
    },
    Shape {
        name: "python_many_defs",
        lang: "python",
        generate: python_defs,
    },
    Shape {
        name: "js_many_fns",
        lang: "javascript",
        generate: js_fns,
    },
    Shape {
        name: "c_many_fns",
        lang: "c",
        generate: c_fns,
    },
    Shape {
        name: "go_many_fns",
        lang: "go",
        generate: go_fns,
    },
    Shape {
        name: "java_many_methods",
        lang: "java",
        generate: java_methods,
    },
    Shape {
        name: "rust_nested_modules",
        lang: "rust",
        generate: rust_nested,
    },
    Shape {
        name: "python_giant_class",
        lang: "python",
        generate: python_giant_class,
    },
    Shape {
        name: "json_array",
        lang: "json",
        generate: json_array,
    },
    Shape {
        name: "json_object",
        lang: "json",
        generate: json_object,
    },
    Shape {
        name: "json_one_line",
        lang: "json",
        generate: json_one_line,
    },
    Shape {
        name: "json_numbers_lines",
        lang: "json",
        generate: json_numbers_newline_separated,
    },
    Shape {
        name: "nested_brackets",
        lang: "json",
        generate: nested_brackets,
    },
    Shape {
        name: "nested_objects",
        lang: "json",
        generate: nested_objects,
    },
    Shape {
        name: "yaml_map",
        lang: "yaml",
        generate: yaml_map,
    },
    Shape {
        name: "yaml_list",
        lang: "yaml",
        generate: yaml_list,
    },
    Shape {
        name: "csv_rows",
        lang: "csv",
        generate: csv_rows,
    },
    Shape {
        name: "markdown_sections",
        lang: "markdown",
        generate: markdown_sections,
    },
    Shape {
        name: "toml_tables",
        lang: "toml",
        generate: toml_tables,
    },
    Shape {
        name: "js_one_huge_line",
        lang: "javascript",
        generate: js_one_huge_line,
    },
    Shape {
        name: "rust_huge_string_line",
        lang: "rust",
        generate: rust_huge_string_line,
    },
    Shape {
        name: "rust_cjk_emoji",
        lang: "rust",
        generate: rust_cjk_emoji,
    },
    Shape {
        name: "python_cjk_emoji",
        lang: "python",
        generate: python_cjk_emoji,
    },
    Shape {
        name: "rust_crlf",
        lang: "rust",
        generate: rust_crlf,
    },
    Shape {
        name: "python_crlf",
        lang: "python",
        generate: python_crlf,
    },
    Shape {
        name: "python_imports",
        lang: "python",
        generate: python_imports,
    },
    Shape {
        name: "python_comments_docstrings",
        lang: "python",
        generate: python_comments_docstrings,
    },
    Shape {
        name: "js_imports_exports",
        lang: "javascript",
        generate: js_exports,
    },
    Shape {
        name: "python_broken",
        lang: "python",
        generate: python_broken,
    },
];

/// Generate `shape` at roughly `target_bytes` with a seed derived from the shape name, so
/// the 1x/2x/4x/8x inputs of one shape share a prefix structure.
pub fn generate(shape: &Shape, target_bytes: usize) -> String {
    let seed = shape.name.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3)
    });
    (shape.generate)(&mut Rng::new(seed), target_bytes)
}
