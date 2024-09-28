
use crate::syn::{SyntaxKind, SyntaxNode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Name == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Literal == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for StringLiteral {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::StringLiteral == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumberLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for NumberLiteral {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::NumberLiteral == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BooleanLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for BooleanLiteral {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::BooleanLiteral == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for NullLiteral {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::NullLiteral == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ArrayLiteral {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ArrayLiteral == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectLiteral {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ObjectLiteral {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ObjectLiteral == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralList {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for LiteralList {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::LiteralList == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralFieldList {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for LiteralFieldList {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::LiteralFieldList == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralField {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for LiteralField {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::LiteralField == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeDecl {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for TypeDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::TypeDecl == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldList {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for FieldList {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::FieldList == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeExpr {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for TypeExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::TypeExpr == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArray {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for TypeArray {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::TypeArray == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Field {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Field == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Macros {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Macros {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Macros == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decorators {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Decorators {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Decorators == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Macro {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Macro {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Macro == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decorator {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Decorator {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Decorator == kind
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
