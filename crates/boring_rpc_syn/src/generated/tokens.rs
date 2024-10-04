
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
pub struct TrueKeyword {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for TrueKeyword {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::TrueKeyword == kind
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
pub struct FalseKeyword {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for FalseKeyword {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::FalseKeyword == kind
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
pub struct NullKeyword {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for NullKeyword {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::NullKeyword == kind
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
pub struct String {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for String {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::String == kind
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
pub struct Number {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Number {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Number == kind
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
pub struct Invalid {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Invalid {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Invalid == kind
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LBracket {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LBracket {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::LBracket == kind
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
pub struct RBracket {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for RBracket {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::RBracket == kind
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
pub struct LParenthesis {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LParenthesis {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::LParenthesis == kind
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
pub struct RParenthesis {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for RParenthesis {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::RParenthesis == kind
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
pub struct Comma {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Comma {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Comma == kind
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
pub struct Hash {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Hash {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Hash == kind
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
pub struct At {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for At {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::At == kind
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
pub struct Colon {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for Colon {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Colon == kind
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
