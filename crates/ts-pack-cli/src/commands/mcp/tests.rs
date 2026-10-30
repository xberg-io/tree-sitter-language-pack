use super::*;

#[test]
fn should_accept_names_on_the_allowlist() {
    for name in ["python", "c_sharp", "html", "tsx", "php2"] {
        assert!(validate_name("language", name).is_ok(), "'{name}' should be accepted");
    }
}

#[test]
fn should_reject_names_outside_the_allowlist() {
    for name in [
        "",
        "Python",
        "../../etc/passwd",
        "py thon",
        "py/thon",
        "py-thon",
        "py.thon",
        "py\0thon",
        "py;rm -rf /",
    ] {
        assert!(validate_name("language", name).is_err(), "'{name}' should be rejected");
    }
    let too_long = "a".repeat(MAX_NAME_LEN + 1);
    assert!(validate_name("language", &too_long).is_err(), "over-long name rejected");
}

#[tokio::test]
async fn should_reject_parse_with_a_traversal_language_name() {
    let server = TsPackMcp::new();
    let result = server
        .parse(Parameters(ParseParams {
            source: "x = 1".to_string(),
            language: "../../etc/passwd".to_string(),
            format: None,
        }))
        .await;
    assert!(result.is_err(), "traversal language name is rejected");
}

#[test]
fn should_reject_resource_uri_with_an_invalid_language_name() {
    let server = TsPackMcp::new();
    assert!(server.read_resource_inner("ts-pack://language/../secrets").is_err());
}

#[test]
fn should_allow_only_loopback_origins_by_default() {
    let guard = HttpGuard::new(8011, None, None);
    assert!(guard.is_origin_allowed("http://127.0.0.1:8011"));
    assert!(guard.is_origin_allowed("http://localhost:8011"));
    assert!(!guard.is_origin_allowed("http://evil.example.com"));
    assert!(!guard.is_origin_allowed("http://127.0.0.1:9999"));
}

#[test]
fn should_allow_extra_configured_origins() {
    let guard = HttpGuard::new(8011, Some("https://ide.example.com, https://other.example.com"), None);
    assert!(guard.is_origin_allowed("https://ide.example.com"));
    assert!(guard.is_origin_allowed("https://other.example.com"));
    assert!(!guard.is_origin_allowed("https://nope.example.com"));
}

#[test]
fn should_accept_any_token_when_none_is_configured() {
    let guard = HttpGuard::new(8011, None, None);
    assert!(guard.is_token_valid(None));
    assert!(guard.is_token_valid(Some("anything")));
}

#[test]
fn should_require_the_exact_token_when_one_is_configured() {
    let guard = HttpGuard::new(8011, None, Some("s3cret".to_string()));
    assert!(guard.is_token_valid(Some("s3cret")));
    assert!(!guard.is_token_valid(Some("s3cre")));
    assert!(!guard.is_token_valid(Some("wrong")));
    assert!(!guard.is_token_valid(None));
}

#[test]
fn should_treat_a_blank_configured_token_as_unset() {
    let guard = HttpGuard::new(8011, None, Some("   ".to_string()));
    assert!(guard.auth_token.is_none());
    assert!(guard.is_token_valid(None));
}

#[test]
fn test_tool_router_has_all_tools() {
    let router = TsPackMcp::tool_router();
    for name in [
        "parse",
        "process",
        "detect_language",
        "list_languages",
        "info",
        "download",
        "cache_dir",
        "clean_cache",
    ] {
        assert!(router.has_route(name), "Expected tool '{name}' to be registered");
    }
}

#[test]
fn test_server_info_fields() {
    let server = TsPackMcp::new();
    let info = server.get_info();

    assert_eq!(info.server_info.name, "ts-pack-mcp");
    assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
    assert!(info.capabilities.tools.is_some(), "tools capability advertised");
    assert!(info.capabilities.resources.is_some(), "resources capability advertised");
    assert!(info.capabilities.prompts.is_some(), "prompts capability advertised");
    assert!(
        info.capabilities.completions.is_some(),
        "completions capability advertised"
    );
    assert!(info.instructions.is_some());
}

#[test]
fn test_list_resources_exposes_catalogs() {
    let server = TsPackMcp::new();
    let result = server.list_resources_inner();
    let uris: Vec<&str> = result.resources.iter().map(|r| r.uri.as_str()).collect();
    assert!(
        uris.contains(&"ts-pack://languages"),
        "available catalog resource present"
    );
    assert!(
        uris.contains(&"ts-pack://languages/downloaded"),
        "downloaded catalog resource present"
    );
}

#[test]
fn test_list_resource_templates_exposes_language_template() {
    let server = TsPackMcp::new();
    let result = server.list_resource_templates_inner();
    assert!(
        result
            .resource_templates
            .iter()
            .any(|t| t.uri_template == "ts-pack://language/{name}"),
        "per-language template present"
    );
}

#[test]
fn test_read_resource_available_languages() {
    let server = TsPackMcp::new();
    let result = server
        .read_resource_inner("ts-pack://languages")
        .expect("read should succeed");
    let ResourceContents::TextResourceContents { text, .. } = &result.contents[0] else {
        panic!("expected text contents");
    };
    let parsed: serde_json::Value = serde_json::from_str(text).expect("valid JSON");
    let count = parsed["count"].as_u64().expect("count is a number");
    let languages = parsed["languages"].as_array().expect("languages is an array");
    assert_eq!(count as usize, languages.len(), "count matches the array length");
}

#[test]
fn test_read_resource_language_template() {
    let server = TsPackMcp::new();
    let result = server
        .read_resource_inner("ts-pack://language/python")
        .expect("read should succeed");
    let ResourceContents::TextResourceContents { text, .. } = &result.contents[0] else {
        panic!("expected text contents");
    };
    let parsed: serde_json::Value = serde_json::from_str(text).expect("valid JSON");
    assert_eq!(parsed["language"], "python");
    assert!(parsed["known"].is_boolean());
}

#[test]
fn test_read_resource_unknown_uri_errors() {
    let server = TsPackMcp::new();
    assert!(server.read_resource_inner("ts-pack://nope").is_err());
}

#[test]
fn test_list_prompts_exposes_analyze_code() {
    let server = TsPackMcp::new();
    let result = server.list_prompts_inner();
    assert!(result.prompts.iter().any(|p| p.name == "analyze-code"));
}

#[test]
fn test_get_prompt_renders_language_and_focus() {
    let server = TsPackMcp::new();
    let mut args = serde_json::Map::new();
    args.insert("language".to_string(), serde_json::json!("rust"));
    args.insert("focus".to_string(), serde_json::json!("security"));
    let result = server
        .get_prompt_inner("analyze-code", Some(args))
        .expect("render should succeed");
    let ContentBlock::Text(text_content) = &result.messages[0].content else {
        panic!("expected text message");
    };
    let text = &text_content.text;
    assert!(text.contains("rust"), "language interpolated");
    assert!(text.contains("security"), "focus interpolated");
}

#[test]
fn test_get_prompt_unknown_errors() {
    let server = TsPackMcp::new();
    assert!(server.get_prompt_inner("nope", None).is_err());
}

#[test]
fn test_complete_language_prefix() {
    let server = TsPackMcp::new();
    let reference = Reference::for_resource("ts-pack://language/{name}");
    let argument = ArgumentInfo::new("name", "py");
    let result = server
        .complete_inner(&reference, &argument)
        .expect("complete should succeed");
    assert!(
        result.completion.values.iter().all(|v| v.starts_with("py")),
        "all completions match the prefix"
    );
    assert!(result.completion.total.is_some(), "completion reports a total count");
}

#[test]
fn test_complete_ignores_unrelated_reference() {
    let server = TsPackMcp::new();
    let reference = Reference::for_prompt("other");
    let argument = ArgumentInfo::new("language", "py");
    let result = server
        .complete_inner(&reference, &argument)
        .expect("complete should succeed");
    assert!(
        result.completion.values.is_empty(),
        "no completions for unrelated prompt"
    );
}

#[tokio::test]
async fn test_list_languages_available() {
    let server = TsPackMcp::new();
    let result = server
        .list_languages(Parameters(ListLanguagesParams {
            source: None,
            filter: None,
        }))
        .await
        .expect("list should succeed");
    assert_eq!(result.0.source, "available");
    assert_eq!(
        result.0.count,
        result.0.languages.len(),
        "count matches the list length"
    );
}

#[tokio::test]
async fn test_list_languages_with_filter() {
    let server = TsPackMcp::new();
    let result = server
        .list_languages(Parameters(ListLanguagesParams {
            source: Some("available".to_string()),
            filter: Some("python".to_string()),
        }))
        .await
        .expect("list should succeed");
    assert_eq!(result.0.filter.as_deref(), Some("python"));
    assert_eq!(result.0.source, "available");
    assert!(
        result.0.languages.iter().all(|l| l.contains("python")),
        "every returned language matches the filter"
    );
}

#[tokio::test]
async fn test_cache_dir_returns_path() {
    let server = TsPackMcp::new();
    let result = server.cache_dir().await.expect("cache_dir should succeed");
    assert!(!result.0.cache_dir.is_empty(), "cache directory path is non-empty");
}

#[tokio::test]
async fn test_detect_language_from_path() {
    let server = TsPackMcp::new();
    let result = server
        .detect_language(Parameters(DetectLanguageParams {
            path: Some("main.py".to_string()),
            content: None,
        }))
        .await
        .expect("detect should succeed");
    assert_eq!(result.0.language.as_deref(), Some("python"));
}

#[tokio::test]
async fn test_download_requires_params() {
    let server = TsPackMcp::new();
    let result = server
        .download(Parameters(DownloadParams {
            languages: None,
            all: None,
            groups: None,
            fresh: None,
        }))
        .await;
    assert!(result.is_err());
}
