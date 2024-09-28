pub use crate::generated::syntax_kinds::SyntaxKind;

pub trait AstToken {
    fn can_cast(kind: SyntaxKind) -> bool;
    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;
    fn syntax(&self) -> &SyntaxToken;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken {}

impl SyntaxToken {
    pub fn kind(&self) -> SyntaxKind {
        SyntaxKind::EOF
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode {}
