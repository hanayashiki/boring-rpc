
use crate::syn::{AstNode, SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Name == kind
    }
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Name {
    pub fn ident(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Literal {
    pub fn string_literal(&self) -> Option<StringLiteral> {
        None
    }
    pub fn null_literal(&self) -> Option<NullLiteral> {
        None
    }
    pub fn array_literal(&self) -> Option<ArrayLiteral> {
        None
    }
    pub fn object_literal(&self) -> Option<ObjectLiteral> {
        None
    }
    pub fn number_literal(&self) -> Option<NumberLiteral> {
        None
    }
    pub fn boolean_literal(&self) -> Option<BooleanLiteral> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl StringLiteral {
    pub fn string(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NumberLiteral {
    pub fn number(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl BooleanLiteral {
    pub fn true_keyword(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn false_keyword(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NullLiteral {
    pub fn null_keyword(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ArrayLiteral {
    pub fn r_bracket(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn literal_list(&self) -> Option<LiteralList> {
        None
    }
    pub fn l_bracket(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ObjectLiteral {
    pub fn literal_field_list(&self) -> Option<LiteralFieldList> {
        None
    }
    pub fn r_curly(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn l_curly(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl LiteralList {
    pub fn comma(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn literal(&self) -> std::vec::Vec<Literal> {
        vec![]
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl LiteralFieldList {
    pub fn literal_field(&self) -> std::vec::Vec<LiteralField> {
        vec![]
    }
    pub fn comma(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl LiteralField {
    pub fn colon(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn key(&self) -> Option<Name> {
        None
    }
    pub fn value(&self) -> Option<Literal> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeDecl {
    pub fn field_list(&self) -> Option<FieldList> {
        None
    }
    pub fn name(&self) -> Option<Name> {
        None
    }
    pub fn r_curly(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn equal(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn l_curly(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn type_keyword(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FieldList {
    pub fn field(&self) -> std::vec::Vec<Field> {
        vec![]
    }
    pub fn comma(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeExpr {
    pub fn name(&self) -> Option<Name> {
        None
    }
    pub fn type_array(&self) -> Option<TypeArray> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeArray {
    pub fn r_bracket(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn type_expr(&self) -> Option<TypeExpr> {
        None
    }
    pub fn l_bracket(&self) -> Option<SyntaxToken> {
        None
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
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Field {
    pub fn field_name(&self) -> Option<Name> {
        None
    }
    pub fn field_type(&self) -> Option<TypeExpr> {
        None
    }
    pub fn colon(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn macro_attrs(&self) -> Option<MacroAttrs> {
        None
    }
    pub fn decorator_attrs(&self) -> Option<DecoratorAttrs> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroAttrs {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for MacroAttrs {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::MacroAttrs == kind
    }
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MacroAttrs {
    pub fn macro_attr(&self) -> std::vec::Vec<MacroAttr> {
        vec![]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecoratorAttrs {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DecoratorAttrs {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::DecoratorAttrs == kind
    }
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl DecoratorAttrs {
    pub fn decorator_attr(&self) -> std::vec::Vec<DecoratorAttr> {
        vec![]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroAttr {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for MacroAttr {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::MacroAttr == kind
    }
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MacroAttr {
    pub fn name(&self) -> Option<Name> {
        None
    }
    pub fn r_parenthesis(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn hash(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn literal_list(&self) -> Option<LiteralList> {
        None
    }
    pub fn l_parenthesis(&self) -> Option<SyntaxToken> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecoratorAttr {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DecoratorAttr {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::DecoratorAttr == kind
    }
    fn cast(_syntax: SyntaxNode) -> Option<Self> {
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl DecoratorAttr {
    pub fn l_parenthesis(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn r_parenthesis(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn name(&self) -> Option<Name> {
        None
    }
    pub fn at(&self) -> Option<SyntaxToken> {
        None
    }
    pub fn literal_list(&self) -> Option<LiteralList> {
        None
    }
}
