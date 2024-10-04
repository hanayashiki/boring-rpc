pub use crate::generated::syntax_kinds::SyntaxKind;

pub trait AstToken {
    fn can_cast(kind: SyntaxKind) -> bool;
    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;
    fn syntax(&self) -> &SyntaxToken;
}

pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool;
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;
    fn syntax(&self) -> &SyntaxNode;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken {
    kind: SyntaxKind,
    value: String,
}

impl SyntaxToken {
    pub fn new(kind: SyntaxKind, value: String) -> Self {
        Self { kind, value }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GreenToken {
    kind: SyntaxKind,
    value: String,
}

impl GreenToken {
    pub fn new(kind: SyntaxKind, value: String) -> Self {
        Self { kind, value }
    }
}
