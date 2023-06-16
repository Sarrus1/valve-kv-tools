use lsp_types::Range;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct KvToken {
    pub(super) text: String,
    pub(super) range: Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TokenKind {
    Key(KvToken),
    Value(KvToken),
    LineComment(KvToken),
    BlockComment(KvToken),
    LBrace,
    RBrace,
}
