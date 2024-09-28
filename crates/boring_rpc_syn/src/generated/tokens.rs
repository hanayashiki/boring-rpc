
use crate::syn::{AstToken, SyntaxKind, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeKeyword {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for TypeKeyword {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::TypeKeyword == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Ident {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Ident == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringLiteral {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for StringLiteral {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::StringLiteral == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumberLiteral {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for NumberLiteral {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::NumberLiteral == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Whitespace {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Whitespace {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Whitespace == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tab {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Tab {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Tab == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Newline {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Newline {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Newline == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EOF {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for EOF {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::EOF == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Equal {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Equal {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Equal == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LCurly {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LCurly {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::LCurly == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RCurly {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for RCurly {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::RCurly == kind
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
