// swift-format-ignore-file
import RustBridgeC

public func parserSetLanguage<GenericIntoRustString: IntoRustString>(_ client: ParserRefMut, _ name: GenericIntoRustString) throws -> () {
    try { let val = __swift_bridge__$parser_set_language(client.ptr, { let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val != nil { throw RustString(ptr: val!) } else { return } }()
}
public func parserParse<GenericIntoRustString: IntoRustString>(_ client: ParserRefMut, _ source: GenericIntoRustString) -> Optional<Tree> {
    { let val = __swift_bridge__$parser_parse(client.ptr, { let rustString = source.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val != nil { return Tree(ptr: val!) } else { return nil } }()
}
public func parserParseBytes(_ client: ParserRefMut, _ source: RustVec<UInt8>) -> Optional<Tree> {
    { let val = __swift_bridge__$parser_parse_bytes(client.ptr, { let val = source; val.isOwned = false; return val.ptr }()); if val != nil { return Tree(ptr: val!) } else { return nil } }()
}
public func parserReset(_ client: ParserRefMut) -> () {
    __swift_bridge__$parser_reset(client.ptr)
}
public func treeRootNode(_ client: TreeRef) -> Node {
    Node(ptr: __swift_bridge__$tree_root_node(client.ptr))
}
public func treeWalk(_ client: TreeRef) -> TreeCursor {
    TreeCursor(ptr: __swift_bridge__$tree_walk(client.ptr))
}
public func nodeClone(_ client: NodeRef) -> Node {
    Node(ptr: __swift_bridge__$node_clone(client.ptr))
}
public func nodeKind(_ client: NodeRef) -> RustString {
    RustString(ptr: __swift_bridge__$node_kind(client.ptr))
}
public func nodeKindId(_ client: NodeRef) -> UInt16 {
    __swift_bridge__$node_kind_id(client.ptr)
}
public func nodeStartByte(_ client: NodeRef) -> UInt {
    __swift_bridge__$node_start_byte(client.ptr)
}
public func nodeEndByte(_ client: NodeRef) -> UInt {
    __swift_bridge__$node_end_byte(client.ptr)
}
public func nodeByteRange(_ client: NodeRef) -> ByteRange {
    ByteRange(ptr: __swift_bridge__$node_byte_range(client.ptr))
}
public func nodeStartPosition(_ client: NodeRef) -> Point {
    Point(ptr: __swift_bridge__$node_start_position(client.ptr))
}
public func nodeEndPosition(_ client: NodeRef) -> Point {
    Point(ptr: __swift_bridge__$node_end_position(client.ptr))
}
public func nodeIsNamed(_ client: NodeRef) -> Bool {
    __swift_bridge__$node_is_named(client.ptr)
}
public func nodeIsError(_ client: NodeRef) -> Bool {
    __swift_bridge__$node_is_error(client.ptr)
}
public func nodeIsMissing(_ client: NodeRef) -> Bool {
    __swift_bridge__$node_is_missing(client.ptr)
}
public func nodeIsExtra(_ client: NodeRef) -> Bool {
    __swift_bridge__$node_is_extra(client.ptr)
}
public func nodeHasError(_ client: NodeRef) -> Bool {
    __swift_bridge__$node_has_error(client.ptr)
}
public func nodeParent(_ client: NodeRef) -> Optional<Node> {
    { let val = __swift_bridge__$node_parent(client.ptr); if val != nil { return Node(ptr: val!) } else { return nil } }()
}
public func nodeChild(_ client: NodeRef, _ index: UInt32) -> Optional<Node> {
    { let val = __swift_bridge__$node_child(client.ptr, index); if val != nil { return Node(ptr: val!) } else { return nil } }()
}
public func nodeChildCount(_ client: NodeRef) -> UInt {
    __swift_bridge__$node_child_count(client.ptr)
}
public func nodeNamedChild(_ client: NodeRef, _ index: UInt32) -> Optional<Node> {
    { let val = __swift_bridge__$node_named_child(client.ptr, index); if val != nil { return Node(ptr: val!) } else { return nil } }()
}
public func nodeNamedChildCount(_ client: NodeRef) -> UInt {
    __swift_bridge__$node_named_child_count(client.ptr)
}
public func nodeChildByFieldName<GenericIntoRustString: IntoRustString>(_ client: NodeRef, _ name: GenericIntoRustString) -> Optional<Node> {
    { let val = __swift_bridge__$node_child_by_field_name(client.ptr, { let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val != nil { return Node(ptr: val!) } else { return nil } }()
}
public func nodeToSexp(_ client: NodeRef) -> RustString {
    RustString(ptr: __swift_bridge__$node_to_sexp(client.ptr))
}
public func nodeWalk(_ client: NodeRef) -> TreeCursor {
    TreeCursor(ptr: __swift_bridge__$node_walk(client.ptr))
}
public func treeCursorNode(_ client: TreeCursorRef) -> Node {
    Node(ptr: __swift_bridge__$tree_cursor_node(client.ptr))
}
public func treeCursorGotoFirstChild(_ client: TreeCursorRefMut) -> Bool {
    __swift_bridge__$tree_cursor_goto_first_child(client.ptr)
}
public func treeCursorGotoParent(_ client: TreeCursorRefMut) -> Bool {
    __swift_bridge__$tree_cursor_goto_parent(client.ptr)
}
public func treeCursorGotoNextSibling(_ client: TreeCursorRefMut) -> Bool {
    __swift_bridge__$tree_cursor_goto_next_sibling(client.ptr)
}
public func treeCursorFieldName(_ client: TreeCursorRef) -> RustString {
    RustString(ptr: __swift_bridge__$tree_cursor_field_name(client.ptr))
}
public func languageRegistryGetLanguage<GenericIntoRustString: IntoRustString>(_ client: LanguageRegistryRef, _ name: GenericIntoRustString) throws -> Language {
    try { let val = __swift_bridge__$language_registry_get_language(client.ptr, { let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Language(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func languageRegistryAvailableLanguages(_ client: LanguageRegistryRef) -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$language_registry_available_languages(client.ptr))
}
public func languageRegistryHasParser<GenericIntoRustString: IntoRustString>(_ client: LanguageRegistryRef, _ name: GenericIntoRustString) -> Bool {
    __swift_bridge__$language_registry_has_parser(client.ptr, { let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
}
public func languageRegistryHasLanguage<GenericIntoRustString: IntoRustString>(_ client: LanguageRegistryRef, _ name: GenericIntoRustString) -> Bool {
    __swift_bridge__$language_registry_has_language(client.ptr, { let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
}
public func languageRegistryLanguageCount(_ client: LanguageRegistryRef) -> UInt {
    __swift_bridge__$language_registry_language_count(client.ptr)
}
public func languageRegistryProcess<GenericIntoRustString: IntoRustString>(_ client: LanguageRegistryRef, _ source: GenericIntoRustString, _ config: ProcessConfig) throws -> ProcessResult {
    try { let val = __swift_bridge__$language_registry_process(client.ptr, { let rustString = source.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {config.isOwned = false; return config.ptr;}()); if val.is_ok { return ProcessResult(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func downloadManagerInstalledLanguages(_ client: DownloadManagerRef) -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$download_manager_installed_languages(client.ptr))
}
public func downloadManagerDownloadAllBestEffort(_ client: DownloadManagerRef) throws -> UInt {
    try { let val = __swift_bridge__$download_manager_download_all_best_effort(client.ptr); switch val.tag { case __swift_bridge__$ResultUIntAndString$ResultOk: return val.payload.ok case __swift_bridge__$ResultUIntAndString$ResultErr: throw RustString(ptr: val.payload.err) default: fatalError() } }()
}
public func downloadManagerCleanCache(_ client: DownloadManagerRef) throws -> () {
    try { let val = __swift_bridge__$download_manager_clean_cache(client.ptr); if val != nil { throw RustString(ptr: val!) } else { return } }()
}
public func detectLanguageFromExtension<GenericIntoRustString: IntoRustString>(_ ext: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$detect_language_from_extension({ let rustString = ext.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func detectLanguageFromPath<GenericIntoRustString: IntoRustString>(_ path: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$detect_language_from_path({ let rustString = path.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func detectLanguageFromContent<GenericIntoRustString: IntoRustString>(_ content: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$detect_language_from_content({ let rustString = content.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func getHighlightsQuery<GenericIntoRustString: IntoRustString>(_ language: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$get_highlights_query({ let rustString = language.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func getInjectionsQuery<GenericIntoRustString: IntoRustString>(_ language: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$get_injections_query({ let rustString = language.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func getLocalsQuery<GenericIntoRustString: IntoRustString>(_ language: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$get_locals_query({ let rustString = language.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func getTagsQuery<GenericIntoRustString: IntoRustString>(_ language: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$get_tags_query({ let rustString = language.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func getLanguage<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString) throws -> Language {
    try { let val = __swift_bridge__$get_language({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Language(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func getParser<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString) throws -> Parser {
    try { let val = __swift_bridge__$get_parser({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Parser(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func detectLanguage<GenericIntoRustString: IntoRustString>(_ path: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$detect_language({ let rustString = path.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func availableLanguages() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$available_languages())
}
public func hasLanguage<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString) -> Bool {
    __swift_bridge__$has_language({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }())
}
public func languageCount() -> UInt {
    __swift_bridge__$language_count()
}
public func process<GenericIntoRustString: IntoRustString>(_ source: GenericIntoRustString, _ config: ProcessConfig) throws -> ProcessResult {
    try { let val = __swift_bridge__$process({ let rustString = source.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {config.isOwned = false; return config.ptr;}()); if val.is_ok { return ProcessResult(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func init_(_ config: PackConfig) throws -> () {
    try { let val = __swift_bridge__$init_({config.isOwned = false; return config.ptr;}()); if val != nil { throw RustString(ptr: val!) } else { return } }()
}
public func configure(_ config: PackConfig) throws -> () {
    try { let val = __swift_bridge__$configure({config.isOwned = false; return config.ptr;}()); if val != nil { throw RustString(ptr: val!) } else { return } }()
}
public func download<GenericIntoRustString: IntoRustString>(_ names: RustVec<GenericIntoRustString>) throws -> UInt {
    try { let val = __swift_bridge__$download({ let val = names; val.isOwned = false; return val.ptr }()); switch val.tag { case __swift_bridge__$ResultUIntAndString$ResultOk: return val.payload.ok case __swift_bridge__$ResultUIntAndString$ResultErr: throw RustString(ptr: val.payload.err) default: fatalError() } }()
}
public func downloadAll() throws -> UInt {
    try { let val = __swift_bridge__$download_all(); switch val.tag { case __swift_bridge__$ResultUIntAndString$ResultOk: return val.payload.ok case __swift_bridge__$ResultUIntAndString$ResultErr: throw RustString(ptr: val.payload.err) default: fatalError() } }()
}
public func downloadGroup<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString) throws -> UInt {
    try { let val = __swift_bridge__$download_group({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); switch val.tag { case __swift_bridge__$ResultUIntAndString$ResultOk: return val.payload.ok case __swift_bridge__$ResultUIntAndString$ResultErr: throw RustString(ptr: val.payload.err) default: fatalError() } }()
}
public func manifestLanguages() throws -> RustVec<RustString> {
    try { let val = __swift_bridge__$manifest_languages(); if val.is_ok { return RustVec(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func downloadedLanguages() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$downloaded_languages())
}
public func cleanCache() throws -> () {
    try { let val = __swift_bridge__$clean_cache(); if val != nil { throw RustString(ptr: val!) } else { return } }()
}
public func cacheDir() throws -> RustString {
    try { let val = __swift_bridge__$cache_dir(); if val.is_ok { return RustString(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func packConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> PackConfig {
    try { let val = __swift_bridge__$pack_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return PackConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func processConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ProcessConfig {
    try { let val = __swift_bridge__$process_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ProcessConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func spanFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Span {
    try { let val = __swift_bridge__$span_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Span(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func processResultFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ProcessResult {
    try { let val = __swift_bridge__$process_result_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ProcessResult(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func fileMetricsFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FileMetrics {
    try { let val = __swift_bridge__$file_metrics_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FileMetrics(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func structureItemFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StructureItem {
    try { let val = __swift_bridge__$structure_item_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StructureItem(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func commentInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CommentInfo {
    try { let val = __swift_bridge__$comment_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CommentInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func docstringInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> DocstringInfo {
    try { let val = __swift_bridge__$docstring_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DocstringInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func docSectionFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> DocSection {
    try { let val = __swift_bridge__$doc_section_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DocSection(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func importInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ImportInfo {
    try { let val = __swift_bridge__$import_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ImportInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func exportInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ExportInfo {
    try { let val = __swift_bridge__$export_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ExportInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func symbolInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SymbolInfo {
    try { let val = __swift_bridge__$symbol_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SymbolInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func diagnosticFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Diagnostic {
    try { let val = __swift_bridge__$diagnostic_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Diagnostic(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func codeChunkFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CodeChunk {
    try { let val = __swift_bridge__$code_chunk_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CodeChunk(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func chunkContextFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ChunkContext {
    try { let val = __swift_bridge__$chunk_context_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ChunkContext(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func pointFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Point {
    try { let val = __swift_bridge__$point_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Point(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func byteRangeFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ByteRange {
    try { let val = __swift_bridge__$byte_range_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ByteRange(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func structureKindFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StructureKind {
    try { let val = __swift_bridge__$structure_kind_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StructureKind(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func commentKindFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CommentKind {
    try { let val = __swift_bridge__$comment_kind_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CommentKind(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func docstringFormatFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> DocstringFormat {
    try { let val = __swift_bridge__$docstring_format_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DocstringFormat(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func exportKindFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ExportKind {
    try { let val = __swift_bridge__$export_kind_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ExportKind(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func symbolKindFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SymbolKind {
    try { let val = __swift_bridge__$symbol_kind_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SymbolKind(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func diagnosticSeverityFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> DiagnosticSeverity {
    try { let val = __swift_bridge__$diagnostic_severity_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return DiagnosticSeverity(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func __alef_phantom_vec_span() -> RustVec<Span> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_span())
}
public func __alef_phantom_vec_process_result() -> RustVec<ProcessResult> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_process_result())
}
public func __alef_phantom_vec_file_metrics() -> RustVec<FileMetrics> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_file_metrics())
}
public func __alef_phantom_vec_structure_item() -> RustVec<StructureItem> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_structure_item())
}
public func __alef_phantom_vec_comment_info() -> RustVec<CommentInfo> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_comment_info())
}
public func __alef_phantom_vec_docstring_info() -> RustVec<DocstringInfo> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_docstring_info())
}
public func __alef_phantom_vec_doc_section() -> RustVec<DocSection> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_doc_section())
}
public func __alef_phantom_vec_import_info() -> RustVec<ImportInfo> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_import_info())
}
public func __alef_phantom_vec_export_info() -> RustVec<ExportInfo> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_export_info())
}
public func __alef_phantom_vec_symbol_info() -> RustVec<SymbolInfo> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_symbol_info())
}
public func __alef_phantom_vec_diagnostic() -> RustVec<Diagnostic> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_diagnostic())
}
public func __alef_phantom_vec_code_chunk() -> RustVec<CodeChunk> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_code_chunk())
}
public func __alef_phantom_vec_chunk_context() -> RustVec<ChunkContext> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_chunk_context())
}
public func __alef_phantom_vec_pack_config() -> RustVec<PackConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_pack_config())
}
public func __alef_phantom_vec_point() -> RustVec<Point> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_point())
}
public func __alef_phantom_vec_byte_range() -> RustVec<ByteRange> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_byte_range())
}
public func __alef_phantom_vec_parser() -> RustVec<Parser> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_parser())
}
public func __alef_phantom_vec_tree() -> RustVec<Tree> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_tree())
}
public func __alef_phantom_vec_node() -> RustVec<Node> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_node())
}
public func __alef_phantom_vec_tree_cursor() -> RustVec<TreeCursor> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_tree_cursor())
}
public func __alef_phantom_vec_process_config() -> RustVec<ProcessConfig> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_process_config())
}
public func __alef_phantom_vec_language_registry() -> RustVec<LanguageRegistry> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_language_registry())
}
public func __alef_phantom_vec_download_manager() -> RustVec<DownloadManager> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_download_manager())
}
public func __alef_phantom_vec_language() -> RustVec<Language> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_language())
}
public func __alef_phantom_vec_structure_kind() -> RustVec<StructureKind> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_structure_kind())
}
public func __alef_phantom_vec_comment_kind() -> RustVec<CommentKind> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_comment_kind())
}
public func __alef_phantom_vec_docstring_format() -> RustVec<DocstringFormat> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_docstring_format())
}
public func __alef_phantom_vec_export_kind() -> RustVec<ExportKind> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_export_kind())
}
public func __alef_phantom_vec_symbol_kind() -> RustVec<SymbolKind> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_symbol_kind())
}
public func __alef_phantom_vec_diagnostic_severity() -> RustVec<DiagnosticSeverity> {
    RustVec(ptr: __swift_bridge__$__alef_phantom_vec_diagnostic_severity())
}

public class Span: SpanRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Span$_free(ptr)
        }
    }
}
extension Span {
    public convenience init(_ start_byte: UInt, _ end_byte: UInt, _ start_line: UInt, _ start_column: UInt, _ end_line: UInt, _ end_column: UInt) {
        self.init(ptr: __swift_bridge__$Span$new(start_byte, end_byte, start_line, start_column, end_line, end_column))
    }
}
public class SpanRefMut: SpanRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SpanRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SpanRef {
    public func startByte() -> UInt {
        __swift_bridge__$Span$start_byte(ptr)
    }

    public func endByte() -> UInt {
        __swift_bridge__$Span$end_byte(ptr)
    }

    public func startLine() -> UInt {
        __swift_bridge__$Span$start_line(ptr)
    }

    public func startColumn() -> UInt {
        __swift_bridge__$Span$start_column(ptr)
    }

    public func endLine() -> UInt {
        __swift_bridge__$Span$end_line(ptr)
    }

    public func endColumn() -> UInt {
        __swift_bridge__$Span$end_column(ptr)
    }
}
extension Span: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Span$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Span$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Span) {
        __swift_bridge__$Vec_Span$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Span$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Span(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SpanRef> {
        let pointer = __swift_bridge__$Vec_Span$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SpanRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SpanRefMut> {
        let pointer = __swift_bridge__$Vec_Span$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SpanRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SpanRef> {
        UnsafePointer<SpanRef>(OpaquePointer(__swift_bridge__$Vec_Span$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Span$len(vecPtr)
    }
}


public class ProcessResult: ProcessResultRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ProcessResult$_free(ptr)
        }
    }
}
extension ProcessResult {
    public convenience init<GenericIntoRustString: IntoRustString>(_ language: GenericIntoRustString, _ metrics: FileMetrics, _ structure: RustVec<StructureItem>, _ imports: RustVec<ImportInfo>, _ exports: RustVec<ExportInfo>, _ comments: RustVec<CommentInfo>, _ docstrings: RustVec<DocstringInfo>, _ symbols: RustVec<SymbolInfo>, _ diagnostics: RustVec<Diagnostic>, _ chunks: RustVec<CodeChunk>) {
        self.init(ptr: __swift_bridge__$ProcessResult$new({ let rustString = language.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {metrics.isOwned = false; return metrics.ptr;}(), { let val = structure; val.isOwned = false; return val.ptr }(), { let val = imports; val.isOwned = false; return val.ptr }(), { let val = exports; val.isOwned = false; return val.ptr }(), { let val = comments; val.isOwned = false; return val.ptr }(), { let val = docstrings; val.isOwned = false; return val.ptr }(), { let val = symbols; val.isOwned = false; return val.ptr }(), { let val = diagnostics; val.isOwned = false; return val.ptr }(), { let val = chunks; val.isOwned = false; return val.ptr }()))
    }
}
public class ProcessResultRefMut: ProcessResultRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ProcessResultRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ProcessResultRef {
    public func language() -> RustString {
        RustString(ptr: __swift_bridge__$ProcessResult$language(ptr))
    }

    public func metrics() -> FileMetrics {
        FileMetrics(ptr: __swift_bridge__$ProcessResult$metrics(ptr))
    }

    public func structure() -> RustVec<StructureItem> {
        RustVec(ptr: __swift_bridge__$ProcessResult$structure(ptr))
    }

    public func imports() -> RustVec<ImportInfo> {
        RustVec(ptr: __swift_bridge__$ProcessResult$imports(ptr))
    }

    public func exports() -> RustVec<ExportInfo> {
        RustVec(ptr: __swift_bridge__$ProcessResult$exports(ptr))
    }

    public func comments() -> RustVec<CommentInfo> {
        RustVec(ptr: __swift_bridge__$ProcessResult$comments(ptr))
    }

    public func docstrings() -> RustVec<DocstringInfo> {
        RustVec(ptr: __swift_bridge__$ProcessResult$docstrings(ptr))
    }

    public func symbols() -> RustVec<SymbolInfo> {
        RustVec(ptr: __swift_bridge__$ProcessResult$symbols(ptr))
    }

    public func diagnostics() -> RustVec<Diagnostic> {
        RustVec(ptr: __swift_bridge__$ProcessResult$diagnostics(ptr))
    }

    public func chunks() -> RustVec<CodeChunk> {
        RustVec(ptr: __swift_bridge__$ProcessResult$chunks(ptr))
    }
}
extension ProcessResult: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ProcessResult$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ProcessResult$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ProcessResult) {
        __swift_bridge__$Vec_ProcessResult$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ProcessResult$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ProcessResult(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProcessResultRef> {
        let pointer = __swift_bridge__$Vec_ProcessResult$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ProcessResultRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProcessResultRefMut> {
        let pointer = __swift_bridge__$Vec_ProcessResult$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ProcessResultRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ProcessResultRef> {
        UnsafePointer<ProcessResultRef>(OpaquePointer(__swift_bridge__$Vec_ProcessResult$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ProcessResult$len(vecPtr)
    }
}


public class FileMetrics: FileMetricsRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$FileMetrics$_free(ptr)
        }
    }
}
extension FileMetrics {
    public convenience init(_ total_lines: UInt, _ code_lines: UInt, _ comment_lines: UInt, _ blank_lines: UInt, _ total_bytes: UInt, _ node_count: UInt, _ error_count: UInt, _ max_depth: UInt) {
        self.init(ptr: __swift_bridge__$FileMetrics$new(total_lines, code_lines, comment_lines, blank_lines, total_bytes, node_count, error_count, max_depth))
    }
}
public class FileMetricsRefMut: FileMetricsRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class FileMetricsRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FileMetricsRef {
    public func totalLines() -> UInt {
        __swift_bridge__$FileMetrics$total_lines(ptr)
    }

    public func codeLines() -> UInt {
        __swift_bridge__$FileMetrics$code_lines(ptr)
    }

    public func commentLines() -> UInt {
        __swift_bridge__$FileMetrics$comment_lines(ptr)
    }

    public func blankLines() -> UInt {
        __swift_bridge__$FileMetrics$blank_lines(ptr)
    }

    public func totalBytes() -> UInt {
        __swift_bridge__$FileMetrics$total_bytes(ptr)
    }

    public func nodeCount() -> UInt {
        __swift_bridge__$FileMetrics$node_count(ptr)
    }

    public func errorCount() -> UInt {
        __swift_bridge__$FileMetrics$error_count(ptr)
    }

    public func maxDepth() -> UInt {
        __swift_bridge__$FileMetrics$max_depth(ptr)
    }
}
extension FileMetrics: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_FileMetrics$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_FileMetrics$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FileMetrics) {
        __swift_bridge__$Vec_FileMetrics$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_FileMetrics$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (FileMetrics(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FileMetricsRef> {
        let pointer = __swift_bridge__$Vec_FileMetrics$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FileMetricsRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FileMetricsRefMut> {
        let pointer = __swift_bridge__$Vec_FileMetrics$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return FileMetricsRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FileMetricsRef> {
        UnsafePointer<FileMetricsRef>(OpaquePointer(__swift_bridge__$Vec_FileMetrics$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_FileMetrics$len(vecPtr)
    }
}


public class StructureItem: StructureItemRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$StructureItem$_free(ptr)
        }
    }
}
extension StructureItem {
    public convenience init<GenericIntoRustString: IntoRustString>(_ kind: StructureKind, _ name: Optional<GenericIntoRustString>, _ visibility: Optional<GenericIntoRustString>, _ span: Span, _ children: RustVec<StructureItem>, _ decorators: RustVec<GenericIntoRustString>, _ doc_comment: Optional<GenericIntoRustString>, _ signature: Optional<GenericIntoRustString>, _ body_span: Optional<Span>) {
        self.init(ptr: __swift_bridge__$StructureItem$new({kind.isOwned = false; return kind.ptr;}(), { if let rustString = optionalStringIntoRustString(name) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(visibility) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), {span.isOwned = false; return span.ptr;}(), { let val = children; val.isOwned = false; return val.ptr }(), { let val = decorators; val.isOwned = false; return val.ptr }(), { if let rustString = optionalStringIntoRustString(doc_comment) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(signature) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let val = body_span { val.isOwned = false; return val.ptr } else { return nil } }()))
    }
}
public class StructureItemRefMut: StructureItemRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class StructureItemRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension StructureItemRef {
    public func kind() -> RustString {
        RustString(ptr: __swift_bridge__$StructureItem$kind(ptr))
    }

    public func name() -> Optional<RustString> {
        { let val = __swift_bridge__$StructureItem$name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func visibility() -> Optional<RustString> {
        { let val = __swift_bridge__$StructureItem$visibility(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func span() -> Span {
        Span(ptr: __swift_bridge__$StructureItem$span(ptr))
    }

    public func children() -> RustVec<StructureItem> {
        RustVec(ptr: __swift_bridge__$StructureItem$children(ptr))
    }

    public func decorators() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$StructureItem$decorators(ptr))
    }

    public func docComment() -> Optional<RustString> {
        { let val = __swift_bridge__$StructureItem$doc_comment(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func signature() -> Optional<RustString> {
        { let val = __swift_bridge__$StructureItem$signature(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func bodySpan() -> Optional<Span> {
        { let val = __swift_bridge__$StructureItem$body_span(ptr); if val != nil { return Span(ptr: val!) } else { return nil } }()
    }
}
extension StructureItem: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_StructureItem$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_StructureItem$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StructureItem) {
        __swift_bridge__$Vec_StructureItem$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_StructureItem$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (StructureItem(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StructureItemRef> {
        let pointer = __swift_bridge__$Vec_StructureItem$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StructureItemRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StructureItemRefMut> {
        let pointer = __swift_bridge__$Vec_StructureItem$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StructureItemRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StructureItemRef> {
        UnsafePointer<StructureItemRef>(OpaquePointer(__swift_bridge__$Vec_StructureItem$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_StructureItem$len(vecPtr)
    }
}


public class CommentInfo: CommentInfoRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CommentInfo$_free(ptr)
        }
    }
}
extension CommentInfo {
    public convenience init<GenericIntoRustString: IntoRustString>(_ text: GenericIntoRustString, _ kind: CommentKind, _ span: Span, _ associated_node: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$CommentInfo$new({ let rustString = text.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {kind.isOwned = false; return kind.ptr;}(), {span.isOwned = false; return span.ptr;}(), { if let rustString = optionalStringIntoRustString(associated_node) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class CommentInfoRefMut: CommentInfoRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CommentInfoRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CommentInfoRef {
    public func text() -> RustString {
        RustString(ptr: __swift_bridge__$CommentInfo$text(ptr))
    }

    public func kind() -> RustString {
        RustString(ptr: __swift_bridge__$CommentInfo$kind(ptr))
    }

    public func span() -> Span {
        Span(ptr: __swift_bridge__$CommentInfo$span(ptr))
    }

    public func associatedNode() -> Optional<RustString> {
        { let val = __swift_bridge__$CommentInfo$associated_node(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension CommentInfo: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CommentInfo$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CommentInfo$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CommentInfo) {
        __swift_bridge__$Vec_CommentInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CommentInfo$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CommentInfo(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CommentInfoRef> {
        let pointer = __swift_bridge__$Vec_CommentInfo$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CommentInfoRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CommentInfoRefMut> {
        let pointer = __swift_bridge__$Vec_CommentInfo$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CommentInfoRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CommentInfoRef> {
        UnsafePointer<CommentInfoRef>(OpaquePointer(__swift_bridge__$Vec_CommentInfo$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CommentInfo$len(vecPtr)
    }
}


public class DocstringInfo: DocstringInfoRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DocstringInfo$_free(ptr)
        }
    }
}
extension DocstringInfo {
    public convenience init<GenericIntoRustString: IntoRustString>(_ text: GenericIntoRustString, _ format: DocstringFormat, _ span: Span, _ associated_item: Optional<GenericIntoRustString>, _ parsed_sections: RustVec<DocSection>) {
        self.init(ptr: __swift_bridge__$DocstringInfo$new({ let rustString = text.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {format.isOwned = false; return format.ptr;}(), {span.isOwned = false; return span.ptr;}(), { if let rustString = optionalStringIntoRustString(associated_item) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { let val = parsed_sections; val.isOwned = false; return val.ptr }()))
    }
}
public class DocstringInfoRefMut: DocstringInfoRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DocstringInfoRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DocstringInfoRef {
    public func text() -> RustString {
        RustString(ptr: __swift_bridge__$DocstringInfo$text(ptr))
    }

    public func format() -> RustString {
        RustString(ptr: __swift_bridge__$DocstringInfo$format(ptr))
    }

    public func span() -> Span {
        Span(ptr: __swift_bridge__$DocstringInfo$span(ptr))
    }

    public func associatedItem() -> Optional<RustString> {
        { let val = __swift_bridge__$DocstringInfo$associated_item(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func parsedSections() -> RustVec<DocSection> {
        RustVec(ptr: __swift_bridge__$DocstringInfo$parsed_sections(ptr))
    }
}
extension DocstringInfo: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DocstringInfo$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DocstringInfo$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DocstringInfo) {
        __swift_bridge__$Vec_DocstringInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DocstringInfo$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DocstringInfo(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DocstringInfoRef> {
        let pointer = __swift_bridge__$Vec_DocstringInfo$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DocstringInfoRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DocstringInfoRefMut> {
        let pointer = __swift_bridge__$Vec_DocstringInfo$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DocstringInfoRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DocstringInfoRef> {
        UnsafePointer<DocstringInfoRef>(OpaquePointer(__swift_bridge__$Vec_DocstringInfo$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DocstringInfo$len(vecPtr)
    }
}


public class DocSection: DocSectionRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DocSection$_free(ptr)
        }
    }
}
extension DocSection {
    public convenience init<GenericIntoRustString: IntoRustString>(_ kind: GenericIntoRustString, _ name: Optional<GenericIntoRustString>, _ description: GenericIntoRustString) {
        self.init(ptr: __swift_bridge__$DocSection$new({ let rustString = kind.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(name) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { let rustString = description.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class DocSectionRefMut: DocSectionRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DocSectionRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DocSectionRef {
    public func kind() -> RustString {
        RustString(ptr: __swift_bridge__$DocSection$kind(ptr))
    }

    public func name() -> Optional<RustString> {
        { let val = __swift_bridge__$DocSection$name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func description() -> RustString {
        RustString(ptr: __swift_bridge__$DocSection$description(ptr))
    }
}
extension DocSection: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DocSection$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DocSection$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DocSection) {
        __swift_bridge__$Vec_DocSection$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DocSection$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DocSection(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DocSectionRef> {
        let pointer = __swift_bridge__$Vec_DocSection$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DocSectionRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DocSectionRefMut> {
        let pointer = __swift_bridge__$Vec_DocSection$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DocSectionRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DocSectionRef> {
        UnsafePointer<DocSectionRef>(OpaquePointer(__swift_bridge__$Vec_DocSection$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DocSection$len(vecPtr)
    }
}


public class ImportInfo: ImportInfoRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ImportInfo$_free(ptr)
        }
    }
}
extension ImportInfo {
    public convenience init<GenericIntoRustString: IntoRustString>(_ source: GenericIntoRustString, _ items: RustVec<GenericIntoRustString>, _ alias: Optional<GenericIntoRustString>, _ is_wildcard: Bool, _ span: Span) {
        self.init(ptr: __swift_bridge__$ImportInfo$new({ let rustString = source.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = items; val.isOwned = false; return val.ptr }(), { if let rustString = optionalStringIntoRustString(alias) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), is_wildcard, {span.isOwned = false; return span.ptr;}()))
    }
}
public class ImportInfoRefMut: ImportInfoRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ImportInfoRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ImportInfoRef {
    public func source() -> RustString {
        RustString(ptr: __swift_bridge__$ImportInfo$source(ptr))
    }

    public func items() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ImportInfo$items(ptr))
    }

    public func alias() -> Optional<RustString> {
        { let val = __swift_bridge__$ImportInfo$alias(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func isWildcard() -> Bool {
        __swift_bridge__$ImportInfo$is_wildcard(ptr)
    }

    public func span() -> Span {
        Span(ptr: __swift_bridge__$ImportInfo$span(ptr))
    }
}
extension ImportInfo: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ImportInfo$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ImportInfo$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ImportInfo) {
        __swift_bridge__$Vec_ImportInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ImportInfo$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ImportInfo(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImportInfoRef> {
        let pointer = __swift_bridge__$Vec_ImportInfo$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImportInfoRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ImportInfoRefMut> {
        let pointer = __swift_bridge__$Vec_ImportInfo$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ImportInfoRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ImportInfoRef> {
        UnsafePointer<ImportInfoRef>(OpaquePointer(__swift_bridge__$Vec_ImportInfo$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ImportInfo$len(vecPtr)
    }
}


public class ExportInfo: ExportInfoRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ExportInfo$_free(ptr)
        }
    }
}
extension ExportInfo {
    public convenience init<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString, _ kind: ExportKind, _ span: Span) {
        self.init(ptr: __swift_bridge__$ExportInfo$new({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {kind.isOwned = false; return kind.ptr;}(), {span.isOwned = false; return span.ptr;}()))
    }
}
public class ExportInfoRefMut: ExportInfoRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ExportInfoRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ExportInfoRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$ExportInfo$name(ptr))
    }

    public func kind() -> RustString {
        RustString(ptr: __swift_bridge__$ExportInfo$kind(ptr))
    }

    public func span() -> Span {
        Span(ptr: __swift_bridge__$ExportInfo$span(ptr))
    }
}
extension ExportInfo: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ExportInfo$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ExportInfo$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ExportInfo) {
        __swift_bridge__$Vec_ExportInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ExportInfo$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ExportInfo(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ExportInfoRef> {
        let pointer = __swift_bridge__$Vec_ExportInfo$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ExportInfoRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ExportInfoRefMut> {
        let pointer = __swift_bridge__$Vec_ExportInfo$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ExportInfoRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ExportInfoRef> {
        UnsafePointer<ExportInfoRef>(OpaquePointer(__swift_bridge__$Vec_ExportInfo$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ExportInfo$len(vecPtr)
    }
}


public class SymbolInfo: SymbolInfoRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SymbolInfo$_free(ptr)
        }
    }
}
extension SymbolInfo {
    public convenience init<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString, _ kind: SymbolKind, _ span: Span, _ type_annotation: Optional<GenericIntoRustString>, _ doc: Optional<GenericIntoRustString>) {
        self.init(ptr: __swift_bridge__$SymbolInfo$new({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {kind.isOwned = false; return kind.ptr;}(), {span.isOwned = false; return span.ptr;}(), { if let rustString = optionalStringIntoRustString(type_annotation) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let rustString = optionalStringIntoRustString(doc) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
    }
}
public class SymbolInfoRefMut: SymbolInfoRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SymbolInfoRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SymbolInfoRef {
    public func name() -> RustString {
        RustString(ptr: __swift_bridge__$SymbolInfo$name(ptr))
    }

    public func kind() -> RustString {
        RustString(ptr: __swift_bridge__$SymbolInfo$kind(ptr))
    }

    public func span() -> Span {
        Span(ptr: __swift_bridge__$SymbolInfo$span(ptr))
    }

    public func typeAnnotation() -> Optional<RustString> {
        { let val = __swift_bridge__$SymbolInfo$type_annotation(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func doc() -> Optional<RustString> {
        { let val = __swift_bridge__$SymbolInfo$doc(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }
}
extension SymbolInfo: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_SymbolInfo$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_SymbolInfo$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SymbolInfo) {
        __swift_bridge__$Vec_SymbolInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_SymbolInfo$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (SymbolInfo(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SymbolInfoRef> {
        let pointer = __swift_bridge__$Vec_SymbolInfo$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SymbolInfoRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SymbolInfoRefMut> {
        let pointer = __swift_bridge__$Vec_SymbolInfo$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SymbolInfoRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SymbolInfoRef> {
        UnsafePointer<SymbolInfoRef>(OpaquePointer(__swift_bridge__$Vec_SymbolInfo$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_SymbolInfo$len(vecPtr)
    }
}


public class Diagnostic: DiagnosticRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Diagnostic$_free(ptr)
        }
    }
}
extension Diagnostic {
    public convenience init<GenericIntoRustString: IntoRustString>(_ message: GenericIntoRustString, _ severity: DiagnosticSeverity, _ span: Span) {
        self.init(ptr: __swift_bridge__$Diagnostic$new({ let rustString = message.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {severity.isOwned = false; return severity.ptr;}(), {span.isOwned = false; return span.ptr;}()))
    }
}
public class DiagnosticRefMut: DiagnosticRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DiagnosticRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DiagnosticRef {
    public func message() -> RustString {
        RustString(ptr: __swift_bridge__$Diagnostic$message(ptr))
    }

    public func severity() -> RustString {
        RustString(ptr: __swift_bridge__$Diagnostic$severity(ptr))
    }

    public func span() -> Span {
        Span(ptr: __swift_bridge__$Diagnostic$span(ptr))
    }
}
extension Diagnostic: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Diagnostic$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Diagnostic$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Diagnostic) {
        __swift_bridge__$Vec_Diagnostic$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Diagnostic$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Diagnostic(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DiagnosticRef> {
        let pointer = __swift_bridge__$Vec_Diagnostic$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DiagnosticRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DiagnosticRefMut> {
        let pointer = __swift_bridge__$Vec_Diagnostic$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DiagnosticRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DiagnosticRef> {
        UnsafePointer<DiagnosticRef>(OpaquePointer(__swift_bridge__$Vec_Diagnostic$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Diagnostic$len(vecPtr)
    }
}


public class CodeChunk: CodeChunkRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CodeChunk$_free(ptr)
        }
    }
}
extension CodeChunk {
    public convenience init<GenericIntoRustString: IntoRustString>(_ content: GenericIntoRustString, _ start_byte: UInt, _ end_byte: UInt, _ start_line: UInt, _ end_line: UInt, _ metadata: ChunkContext) {
        self.init(ptr: __swift_bridge__$CodeChunk$new({ let rustString = content.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), start_byte, end_byte, start_line, end_line, {metadata.isOwned = false; return metadata.ptr;}()))
    }
}
public class CodeChunkRefMut: CodeChunkRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CodeChunkRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CodeChunkRef {
    public func content() -> RustString {
        RustString(ptr: __swift_bridge__$CodeChunk$content(ptr))
    }

    public func startByte() -> UInt {
        __swift_bridge__$CodeChunk$start_byte(ptr)
    }

    public func endByte() -> UInt {
        __swift_bridge__$CodeChunk$end_byte(ptr)
    }

    public func startLine() -> UInt {
        __swift_bridge__$CodeChunk$start_line(ptr)
    }

    public func endLine() -> UInt {
        __swift_bridge__$CodeChunk$end_line(ptr)
    }

    public func metadata() -> ChunkContext {
        ChunkContext(ptr: __swift_bridge__$CodeChunk$metadata(ptr))
    }
}
extension CodeChunk: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CodeChunk$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CodeChunk$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CodeChunk) {
        __swift_bridge__$Vec_CodeChunk$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CodeChunk$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CodeChunk(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CodeChunkRef> {
        let pointer = __swift_bridge__$Vec_CodeChunk$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CodeChunkRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CodeChunkRefMut> {
        let pointer = __swift_bridge__$Vec_CodeChunk$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CodeChunkRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CodeChunkRef> {
        UnsafePointer<CodeChunkRef>(OpaquePointer(__swift_bridge__$Vec_CodeChunk$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CodeChunk$len(vecPtr)
    }
}


public class ChunkContext: ChunkContextRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ChunkContext$_free(ptr)
        }
    }
}
extension ChunkContext {
    public convenience init<GenericIntoRustString: IntoRustString>(_ language: GenericIntoRustString, _ chunk_index: UInt, _ total_chunks: UInt, _ node_types: RustVec<GenericIntoRustString>, _ context_path: RustVec<GenericIntoRustString>, _ symbols_defined: RustVec<GenericIntoRustString>, _ comments: RustVec<CommentInfo>, _ docstrings: RustVec<DocstringInfo>, _ has_error_nodes: Bool) {
        self.init(ptr: __swift_bridge__$ChunkContext$new({ let rustString = language.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), chunk_index, total_chunks, { let val = node_types; val.isOwned = false; return val.ptr }(), { let val = context_path; val.isOwned = false; return val.ptr }(), { let val = symbols_defined; val.isOwned = false; return val.ptr }(), { let val = comments; val.isOwned = false; return val.ptr }(), { let val = docstrings; val.isOwned = false; return val.ptr }(), has_error_nodes))
    }
}
public class ChunkContextRefMut: ChunkContextRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ChunkContextRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ChunkContextRef {
    public func language() -> RustString {
        RustString(ptr: __swift_bridge__$ChunkContext$language(ptr))
    }

    public func chunkIndex() -> UInt {
        __swift_bridge__$ChunkContext$chunk_index(ptr)
    }

    public func totalChunks() -> UInt {
        __swift_bridge__$ChunkContext$total_chunks(ptr)
    }

    public func nodeTypes() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ChunkContext$node_types(ptr))
    }

    public func contextPath() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ChunkContext$context_path(ptr))
    }

    public func symbolsDefined() -> RustVec<RustString> {
        RustVec(ptr: __swift_bridge__$ChunkContext$symbols_defined(ptr))
    }

    public func comments() -> RustVec<CommentInfo> {
        RustVec(ptr: __swift_bridge__$ChunkContext$comments(ptr))
    }

    public func docstrings() -> RustVec<DocstringInfo> {
        RustVec(ptr: __swift_bridge__$ChunkContext$docstrings(ptr))
    }

    public func hasErrorNodes() -> Bool {
        __swift_bridge__$ChunkContext$has_error_nodes(ptr)
    }
}
extension ChunkContext: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ChunkContext$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ChunkContext$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ChunkContext) {
        __swift_bridge__$Vec_ChunkContext$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ChunkContext$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ChunkContext(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChunkContextRef> {
        let pointer = __swift_bridge__$Vec_ChunkContext$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChunkContextRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ChunkContextRefMut> {
        let pointer = __swift_bridge__$Vec_ChunkContext$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ChunkContextRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ChunkContextRef> {
        UnsafePointer<ChunkContextRef>(OpaquePointer(__swift_bridge__$Vec_ChunkContext$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ChunkContext$len(vecPtr)
    }
}


public class PackConfig: PackConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$PackConfig$_free(ptr)
        }
    }
}
extension PackConfig {
    public convenience init<GenericIntoRustString: IntoRustString>(_ cache_dir: Optional<GenericIntoRustString>, _ languages: Optional<RustVec<GenericIntoRustString>>, _ groups: Optional<RustVec<GenericIntoRustString>>) {
        self.init(ptr: __swift_bridge__$PackConfig$new({ if let rustString = optionalStringIntoRustString(cache_dir) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { if let val = languages { val.isOwned = false; return val.ptr } else { return nil } }(), { if let val = groups { val.isOwned = false; return val.ptr } else { return nil } }()))
    }
}
public class PackConfigRefMut: PackConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class PackConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension PackConfigRef {
    public func cacheDir() -> Optional<RustString> {
        { let val = __swift_bridge__$PackConfig$cache_dir(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
    }

    public func languages() -> Optional<RustVec<RustString>> {
        { let val = __swift_bridge__$PackConfig$languages(ptr); if val != nil { return RustVec(ptr: val!) } else { return nil } }()
    }

    public func groups() -> Optional<RustVec<RustString>> {
        { let val = __swift_bridge__$PackConfig$groups(ptr); if val != nil { return RustVec(ptr: val!) } else { return nil } }()
    }
}
extension PackConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_PackConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_PackConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: PackConfig) {
        __swift_bridge__$Vec_PackConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_PackConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (PackConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PackConfigRef> {
        let pointer = __swift_bridge__$Vec_PackConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PackConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PackConfigRefMut> {
        let pointer = __swift_bridge__$Vec_PackConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PackConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<PackConfigRef> {
        UnsafePointer<PackConfigRef>(OpaquePointer(__swift_bridge__$Vec_PackConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_PackConfig$len(vecPtr)
    }
}


public class Point: PointRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Point$_free(ptr)
        }
    }
}
extension Point {
    public convenience init(_ row: UInt, _ column: UInt) {
        self.init(ptr: __swift_bridge__$Point$new(row, column))
    }
}
public class PointRefMut: PointRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class PointRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension PointRef {
    public func row() -> UInt {
        __swift_bridge__$Point$row(ptr)
    }

    public func column() -> UInt {
        __swift_bridge__$Point$column(ptr)
    }
}
extension Point: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Point$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Point$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Point) {
        __swift_bridge__$Vec_Point$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Point$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Point(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PointRef> {
        let pointer = __swift_bridge__$Vec_Point$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PointRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PointRefMut> {
        let pointer = __swift_bridge__$Vec_Point$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PointRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<PointRef> {
        UnsafePointer<PointRef>(OpaquePointer(__swift_bridge__$Vec_Point$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Point$len(vecPtr)
    }
}


public class ByteRange: ByteRangeRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ByteRange$_free(ptr)
        }
    }
}
extension ByteRange {
    public convenience init(_ start: UInt, _ end: UInt) {
        self.init(ptr: __swift_bridge__$ByteRange$new(start, end))
    }
}
public class ByteRangeRefMut: ByteRangeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ByteRangeRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ByteRangeRef {
    public func start() -> UInt {
        __swift_bridge__$ByteRange$start(ptr)
    }

    public func end() -> UInt {
        __swift_bridge__$ByteRange$end(ptr)
    }
}
extension ByteRange: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ByteRange$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ByteRange$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ByteRange) {
        __swift_bridge__$Vec_ByteRange$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ByteRange$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ByteRange(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ByteRangeRef> {
        let pointer = __swift_bridge__$Vec_ByteRange$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ByteRangeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ByteRangeRefMut> {
        let pointer = __swift_bridge__$Vec_ByteRange$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ByteRangeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ByteRangeRef> {
        UnsafePointer<ByteRangeRef>(OpaquePointer(__swift_bridge__$Vec_ByteRange$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ByteRange$len(vecPtr)
    }
}


public class Parser: ParserRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Parser$_free(ptr)
        }
    }
}
public class ParserRefMut: ParserRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ParserRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension Parser: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Parser$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Parser$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Parser) {
        __swift_bridge__$Vec_Parser$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Parser$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Parser(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParserRef> {
        let pointer = __swift_bridge__$Vec_Parser$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ParserRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParserRefMut> {
        let pointer = __swift_bridge__$Vec_Parser$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ParserRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ParserRef> {
        UnsafePointer<ParserRef>(OpaquePointer(__swift_bridge__$Vec_Parser$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Parser$len(vecPtr)
    }
}


public class Tree: TreeRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Tree$_free(ptr)
        }
    }
}
public class TreeRefMut: TreeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class TreeRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension Tree: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Tree$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Tree$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Tree) {
        __swift_bridge__$Vec_Tree$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Tree$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Tree(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TreeRef> {
        let pointer = __swift_bridge__$Vec_Tree$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return TreeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TreeRefMut> {
        let pointer = __swift_bridge__$Vec_Tree$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return TreeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<TreeRef> {
        UnsafePointer<TreeRef>(OpaquePointer(__swift_bridge__$Vec_Tree$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Tree$len(vecPtr)
    }
}


public class Node: NodeRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Node$_free(ptr)
        }
    }
}
public class NodeRefMut: NodeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class NodeRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension Node: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Node$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Node$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Node) {
        __swift_bridge__$Vec_Node$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Node$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Node(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<NodeRef> {
        let pointer = __swift_bridge__$Vec_Node$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return NodeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<NodeRefMut> {
        let pointer = __swift_bridge__$Vec_Node$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return NodeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<NodeRef> {
        UnsafePointer<NodeRef>(OpaquePointer(__swift_bridge__$Vec_Node$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Node$len(vecPtr)
    }
}


public class TreeCursor: TreeCursorRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$TreeCursor$_free(ptr)
        }
    }
}
public class TreeCursorRefMut: TreeCursorRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class TreeCursorRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension TreeCursor: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_TreeCursor$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_TreeCursor$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: TreeCursor) {
        __swift_bridge__$Vec_TreeCursor$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_TreeCursor$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (TreeCursor(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TreeCursorRef> {
        let pointer = __swift_bridge__$Vec_TreeCursor$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return TreeCursorRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TreeCursorRefMut> {
        let pointer = __swift_bridge__$Vec_TreeCursor$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return TreeCursorRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<TreeCursorRef> {
        UnsafePointer<TreeCursorRef>(OpaquePointer(__swift_bridge__$Vec_TreeCursor$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_TreeCursor$len(vecPtr)
    }
}


public class ProcessConfig: ProcessConfigRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ProcessConfig$_free(ptr)
        }
    }
}
extension ProcessConfig {
    public convenience init<GenericIntoRustString: IntoRustString>(_ language: GenericIntoRustString, _ structure: Bool, _ imports: Bool, _ exports: Bool, _ comments: Bool, _ docstrings: Bool, _ symbols: Bool, _ diagnostics: Bool, _ chunk_max_size: Optional<UInt>) {
        self.init(ptr: __swift_bridge__$ProcessConfig$new({ let rustString = language.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), structure, imports, exports, comments, docstrings, symbols, diagnostics, chunk_max_size.intoFfiRepr()))
    }
}
public class ProcessConfigRefMut: ProcessConfigRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ProcessConfigRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ProcessConfigRef {
    public func language() -> RustString {
        RustString(ptr: __swift_bridge__$ProcessConfig$language(ptr))
    }

    public func structure() -> Bool {
        __swift_bridge__$ProcessConfig$structure(ptr)
    }

    public func imports() -> Bool {
        __swift_bridge__$ProcessConfig$imports(ptr)
    }

    public func exports() -> Bool {
        __swift_bridge__$ProcessConfig$exports(ptr)
    }

    public func comments() -> Bool {
        __swift_bridge__$ProcessConfig$comments(ptr)
    }

    public func docstrings() -> Bool {
        __swift_bridge__$ProcessConfig$docstrings(ptr)
    }

    public func symbols() -> Bool {
        __swift_bridge__$ProcessConfig$symbols(ptr)
    }

    public func diagnostics() -> Bool {
        __swift_bridge__$ProcessConfig$diagnostics(ptr)
    }

    public func chunkMaxSize() -> Optional<UInt> {
        __swift_bridge__$ProcessConfig$chunk_max_size(ptr).intoSwiftRepr()
    }
}
extension ProcessConfig: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ProcessConfig$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ProcessConfig$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ProcessConfig) {
        __swift_bridge__$Vec_ProcessConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ProcessConfig$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ProcessConfig(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProcessConfigRef> {
        let pointer = __swift_bridge__$Vec_ProcessConfig$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ProcessConfigRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProcessConfigRefMut> {
        let pointer = __swift_bridge__$Vec_ProcessConfig$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ProcessConfigRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ProcessConfigRef> {
        UnsafePointer<ProcessConfigRef>(OpaquePointer(__swift_bridge__$Vec_ProcessConfig$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ProcessConfig$len(vecPtr)
    }
}


public class LanguageRegistry: LanguageRegistryRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$LanguageRegistry$_free(ptr)
        }
    }
}
public class LanguageRegistryRefMut: LanguageRegistryRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class LanguageRegistryRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension LanguageRegistry: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_LanguageRegistry$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_LanguageRegistry$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LanguageRegistry) {
        __swift_bridge__$Vec_LanguageRegistry$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_LanguageRegistry$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (LanguageRegistry(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LanguageRegistryRef> {
        let pointer = __swift_bridge__$Vec_LanguageRegistry$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LanguageRegistryRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LanguageRegistryRefMut> {
        let pointer = __swift_bridge__$Vec_LanguageRegistry$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LanguageRegistryRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LanguageRegistryRef> {
        UnsafePointer<LanguageRegistryRef>(OpaquePointer(__swift_bridge__$Vec_LanguageRegistry$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_LanguageRegistry$len(vecPtr)
    }
}


public class DownloadManager: DownloadManagerRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DownloadManager$_free(ptr)
        }
    }
}
public class DownloadManagerRefMut: DownloadManagerRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DownloadManagerRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DownloadManager: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DownloadManager$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DownloadManager$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DownloadManager) {
        __swift_bridge__$Vec_DownloadManager$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DownloadManager$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DownloadManager(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DownloadManagerRef> {
        let pointer = __swift_bridge__$Vec_DownloadManager$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DownloadManagerRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DownloadManagerRefMut> {
        let pointer = __swift_bridge__$Vec_DownloadManager$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DownloadManagerRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DownloadManagerRef> {
        UnsafePointer<DownloadManagerRef>(OpaquePointer(__swift_bridge__$Vec_DownloadManager$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DownloadManager$len(vecPtr)
    }
}


public class Language: LanguageRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Language$_free(ptr)
        }
    }
}
public class LanguageRefMut: LanguageRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class LanguageRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension Language: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Language$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Language$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Language) {
        __swift_bridge__$Vec_Language$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Language$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Language(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LanguageRef> {
        let pointer = __swift_bridge__$Vec_Language$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LanguageRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LanguageRefMut> {
        let pointer = __swift_bridge__$Vec_Language$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LanguageRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LanguageRef> {
        UnsafePointer<LanguageRef>(OpaquePointer(__swift_bridge__$Vec_Language$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Language$len(vecPtr)
    }
}


public class StructureKind: StructureKindRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$StructureKind$_free(ptr)
        }
    }
}
public class StructureKindRefMut: StructureKindRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class StructureKindRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension StructureKindRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$StructureKind$to_string(ptr))
    }
}
extension StructureKind: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_StructureKind$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_StructureKind$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StructureKind) {
        __swift_bridge__$Vec_StructureKind$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_StructureKind$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (StructureKind(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StructureKindRef> {
        let pointer = __swift_bridge__$Vec_StructureKind$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StructureKindRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StructureKindRefMut> {
        let pointer = __swift_bridge__$Vec_StructureKind$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return StructureKindRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StructureKindRef> {
        UnsafePointer<StructureKindRef>(OpaquePointer(__swift_bridge__$Vec_StructureKind$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_StructureKind$len(vecPtr)
    }
}


public class CommentKind: CommentKindRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$CommentKind$_free(ptr)
        }
    }
}
public class CommentKindRefMut: CommentKindRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class CommentKindRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension CommentKindRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$CommentKind$to_string(ptr))
    }
}
extension CommentKind: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_CommentKind$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_CommentKind$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CommentKind) {
        __swift_bridge__$Vec_CommentKind$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_CommentKind$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (CommentKind(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CommentKindRef> {
        let pointer = __swift_bridge__$Vec_CommentKind$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CommentKindRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CommentKindRefMut> {
        let pointer = __swift_bridge__$Vec_CommentKind$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return CommentKindRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CommentKindRef> {
        UnsafePointer<CommentKindRef>(OpaquePointer(__swift_bridge__$Vec_CommentKind$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_CommentKind$len(vecPtr)
    }
}


public class DocstringFormat: DocstringFormatRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DocstringFormat$_free(ptr)
        }
    }
}
public class DocstringFormatRefMut: DocstringFormatRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DocstringFormatRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DocstringFormatRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$DocstringFormat$to_string(ptr))
    }
}
extension DocstringFormat: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DocstringFormat$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DocstringFormat$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DocstringFormat) {
        __swift_bridge__$Vec_DocstringFormat$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DocstringFormat$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DocstringFormat(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DocstringFormatRef> {
        let pointer = __swift_bridge__$Vec_DocstringFormat$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DocstringFormatRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DocstringFormatRefMut> {
        let pointer = __swift_bridge__$Vec_DocstringFormat$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DocstringFormatRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DocstringFormatRef> {
        UnsafePointer<DocstringFormatRef>(OpaquePointer(__swift_bridge__$Vec_DocstringFormat$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DocstringFormat$len(vecPtr)
    }
}


public class ExportKind: ExportKindRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ExportKind$_free(ptr)
        }
    }
}
public class ExportKindRefMut: ExportKindRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ExportKindRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ExportKindRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$ExportKind$to_string(ptr))
    }
}
extension ExportKind: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ExportKind$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ExportKind$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ExportKind) {
        __swift_bridge__$Vec_ExportKind$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ExportKind$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ExportKind(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ExportKindRef> {
        let pointer = __swift_bridge__$Vec_ExportKind$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ExportKindRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ExportKindRefMut> {
        let pointer = __swift_bridge__$Vec_ExportKind$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ExportKindRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ExportKindRef> {
        UnsafePointer<ExportKindRef>(OpaquePointer(__swift_bridge__$Vec_ExportKind$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ExportKind$len(vecPtr)
    }
}


public class SymbolKind: SymbolKindRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SymbolKind$_free(ptr)
        }
    }
}
public class SymbolKindRefMut: SymbolKindRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SymbolKindRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SymbolKindRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$SymbolKind$to_string(ptr))
    }
}
extension SymbolKind: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_SymbolKind$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_SymbolKind$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SymbolKind) {
        __swift_bridge__$Vec_SymbolKind$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_SymbolKind$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (SymbolKind(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SymbolKindRef> {
        let pointer = __swift_bridge__$Vec_SymbolKind$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SymbolKindRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SymbolKindRefMut> {
        let pointer = __swift_bridge__$Vec_SymbolKind$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return SymbolKindRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SymbolKindRef> {
        UnsafePointer<SymbolKindRef>(OpaquePointer(__swift_bridge__$Vec_SymbolKind$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_SymbolKind$len(vecPtr)
    }
}


public class DiagnosticSeverity: DiagnosticSeverityRefMut {
    public var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$DiagnosticSeverity$_free(ptr)
        }
    }
}
public class DiagnosticSeverityRefMut: DiagnosticSeverityRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class DiagnosticSeverityRef {
    public var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension DiagnosticSeverityRef {
    public func to_string() -> RustString {
        RustString(ptr: __swift_bridge__$DiagnosticSeverity$to_string(ptr))
    }
}
extension DiagnosticSeverity: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_DiagnosticSeverity$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_DiagnosticSeverity$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DiagnosticSeverity) {
        __swift_bridge__$Vec_DiagnosticSeverity$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_DiagnosticSeverity$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (DiagnosticSeverity(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DiagnosticSeverityRef> {
        let pointer = __swift_bridge__$Vec_DiagnosticSeverity$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DiagnosticSeverityRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<DiagnosticSeverityRefMut> {
        let pointer = __swift_bridge__$Vec_DiagnosticSeverity$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return DiagnosticSeverityRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<DiagnosticSeverityRef> {
        UnsafePointer<DiagnosticSeverityRef>(OpaquePointer(__swift_bridge__$Vec_DiagnosticSeverity$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_DiagnosticSeverity$len(vecPtr)
    }
}
