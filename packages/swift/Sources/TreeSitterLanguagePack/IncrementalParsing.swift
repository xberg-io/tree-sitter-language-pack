import RustBridge

extension Parser {
    /// Parse UTF-8 source into a new tree.
    public func parse(_ source: String) -> Tree? {
        RustBridge.parserParse(self, source)
    }

    /// Reparse UTF-8 source using a previous tree edited to match it.
    public func parseIncremental(_ source: String, previousTree: Tree) -> Tree? {
        RustBridge.parserParseIncremental(self, source, previousTree)
    }
}

extension Tree {
    /// Return a tree copy with the supplied edit applied.
    public func edited(_ edit: InputEdit) throws -> Tree {
        try alefBridgedCall { try RustBridge.treeEdited(self, edit.intoRust()) }
    }
}
