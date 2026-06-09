package tspack

// NodeInfo contains information about a tree-sitter syntax node.
type NodeInfo struct {
	Kind            string `json:"kind"`
	IsNamed         bool   `json:"is_named"`
	StartByte       int    `json:"start_byte"`
	EndByte         int    `json:"end_byte"`
	StartRow        int    `json:"start_row"`
	StartColumn     int    `json:"start_column"`
	EndRow          int    `json:"end_row"`
	EndColumn       int    `json:"end_column"`
	NamedChildCount int    `json:"named_child_count"`
	IsError         bool   `json:"is_error"`
	IsMissing       bool   `json:"is_missing"`
}

// QueryCapture is a single capture within a query match.
type QueryCapture struct {
	Name string   `json:"name"`
	Node NodeInfo `json:"node"`
}

// QueryMatch is a single match from a tree-sitter query.
type QueryMatch struct {
	PatternIndex int            `json:"pattern_index"`
	Captures     []QueryCapture `json:"captures"`
}

// Span represents a byte and line/column range in source code.
type Span struct {
	StartByte   int `json:"start_byte"`
	EndByte     int `json:"end_byte"`
	StartLine   int `json:"start_line"`
	StartColumn int `json:"start_column"`
	EndLine     int `json:"end_line"`
	EndColumn   int `json:"end_column"`
}

// FileMetrics contains aggregate metrics for a source file.
type FileMetrics struct {
	TotalLines   int `json:"total_lines"`
	CodeLines    int `json:"code_lines"`
	CommentLines int `json:"comment_lines"`
	BlankLines   int `json:"blank_lines"`
	TotalBytes   int `json:"total_bytes"`
	NodeCount    int `json:"node_count"`
	ErrorCount   int `json:"error_count"`
	MaxDepth     int `json:"max_depth"`
}

// StructureItem represents a structural element in source code.
type StructureItem struct {
	Kind       string          `json:"kind"`
	Name       *string         `json:"name,omitempty"`
	Visibility *string         `json:"visibility,omitempty"`
	Span       Span            `json:"span"`
	Children   []StructureItem `json:"children,omitempty"`
	Decorators []string        `json:"decorators,omitempty"`
	DocComment *string         `json:"doc_comment,omitempty"`
	Signature  *string         `json:"signature,omitempty"`
	BodySpan   *Span           `json:"body_span,omitempty"`
}

// ImportInfo represents an import statement.
type ImportInfo struct {
	Source     string   `json:"source"`
	Items      []string `json:"items,omitempty"`
	Alias      *string  `json:"alias,omitempty"`
	IsWildcard bool     `json:"is_wildcard"`
	Span       Span     `json:"span"`
}

// ExportInfo represents an export statement.
type ExportInfo struct {
	Name string `json:"name"`
	Kind string `json:"kind"`
	Span Span   `json:"span"`
}

// CommentInfo represents a comment in source code.
type CommentInfo struct {
	Text           string  `json:"text"`
	Kind           string  `json:"kind"`
	Span           Span    `json:"span"`
	AssociatedNode *string `json:"associated_node,omitempty"`
}

// DocSection represents a section within a docstring.
type DocSection struct {
	Kind        string  `json:"kind"`
	Name        *string `json:"name"`
	Description string  `json:"description"`
}

// DocstringInfo represents a docstring in source code.
type DocstringInfo struct {
	Text           string       `json:"text"`
	Format         string       `json:"format"`
	Span           Span         `json:"span"`
	AssociatedItem *string      `json:"associated_item,omitempty"`
	ParsedSections []DocSection `json:"parsed_sections,omitempty"`
}

// SymbolInfo represents a symbol (variable, function, type) in source code.
type SymbolInfo struct {
	Name           string  `json:"name"`
	Kind           string  `json:"kind"`
	Span           Span    `json:"span"`
	TypeAnnotation *string `json:"type_annotation,omitempty"`
	Doc            *string `json:"doc,omitempty"`
}

// Diagnostic represents a parsing diagnostic (error, warning).
type Diagnostic struct {
	Message  string `json:"message"`
	Severity string `json:"severity"`
	Span     Span   `json:"span"`
}

// FileMetadata contains complete metadata for a source file.
type FileMetadata struct {
	Language    string          `json:"language"`
	Metrics     FileMetrics     `json:"metrics"`
	Structure   []StructureItem `json:"structure,omitempty"`
	Imports     []ImportInfo    `json:"imports,omitempty"`
	Exports     []ExportInfo    `json:"exports,omitempty"`
	Comments    []CommentInfo   `json:"comments,omitempty"`
	Docstrings  []DocstringInfo `json:"docstrings,omitempty"`
	Symbols     []SymbolInfo    `json:"symbols,omitempty"`
	Diagnostics []Diagnostic    `json:"diagnostics,omitempty"`
}

// ChunkInfo contains metadata for a code chunk.
type ChunkInfo struct {
	Language       string          `json:"language"`
	ChunkIndex     int             `json:"chunk_index"`
	TotalChunks    int             `json:"total_chunks"`
	NodeTypes      []string        `json:"node_types"`
	ContextPath    []string        `json:"context_path"`
	SymbolsDefined []string        `json:"symbols_defined"`
	Comments       []CommentInfo   `json:"comments"`
	Docstrings     []DocstringInfo `json:"docstrings"`
	HasErrorNodes  bool            `json:"has_error_nodes"`
}

// CodeChunk is a chunk of source code with rich metadata.
type CodeChunk struct {
	Content   string    `json:"content"`
	StartByte int       `json:"start_byte"`
	EndByte   int       `json:"end_byte"`
	StartLine int       `json:"start_line"`
	EndLine   int       `json:"end_line"`
	Metadata  ChunkInfo `json:"metadata"`
}

// ProcessResult holds the result of Process (analyze + chunk).
type ProcessResult struct {
	Metadata FileMetadata `json:"metadata"`
	Chunks   []CodeChunk  `json:"chunks"`
}

// ProcessConfig specifies what to extract from source code.
type ProcessConfig struct {
	Language    string `json:"language"`
	Structure   bool   `json:"structure"`
	Imports     bool   `json:"imports"`
	Exports     bool   `json:"exports"`
	Comments    bool   `json:"comments"`
	Docstrings  bool   `json:"docstrings"`
	Symbols     bool   `json:"symbols"`
	Diagnostics bool   `json:"diagnostics"`
	// ChunkMaxSize controls chunking. When nil, no chunking is performed.
	ChunkMaxSize *int `json:"chunk_max_size,omitempty"`
}

// NewProcessConfig creates a ProcessConfig with all extraction options enabled
// and no chunking. Pass the language name (e.g. "python", "rust").
func NewProcessConfig(language string) ProcessConfig {
	return ProcessConfig{
		Language:    language,
		Structure:   true,
		Imports:     true,
		Exports:     true,
		Comments:    true,
		Docstrings:  true,
		Symbols:     true,
		Diagnostics: true,
	}
}
