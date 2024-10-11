use std::path::Display;

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

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn with_kind(&self, kind: SyntaxKind) -> GreenToken {
        return GreenToken::new(kind, self.value.clone());
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GreenNode {
    kind: SyntaxKind,
    children: Vec<GreenNodeOrToken>,
}

impl GreenNode {
    pub fn new(kind: SyntaxKind, children: Vec<GreenNodeOrToken>) -> Self {
        Self { kind, children }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn push(&mut self, child: GreenNodeOrToken) {
        self.children.push(child);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GreenNodeOrToken {
    Node(GreenNode),
    Token(GreenToken),
}

impl GreenNodeOrToken {
    fn fmt_impl(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        match self {
            GreenNodeOrToken::Node(node) => {
                writeln!(f, "{}{:?}", String::from(" ").repeat(indent * 4), node.kind)?;
                for child in &node.children {
                    child.fmt_impl(f, indent + 1)?;
                }
            }
            GreenNodeOrToken::Token(token) => {
                writeln!(
                    f,
                    "{}{:?} {}",
                    String::from(" ").repeat(indent * 4),
                    token.kind,
                    token.value
                )?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for GreenNodeOrToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_impl(f, 0)
    }
}
