
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Name {
    pub fn ident(&self) -> Option<SyntaxToken> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Literal {
    pub fn array_literal(&self) -> Option<ArrayLiteral> {
        todo![]
    }
    pub fn boolean_literal(&self) -> Option<BooleanLiteral> {
        todo![]
    }
    pub fn null_literal(&self) -> Option<NullLiteral> {
        todo![]
    }
    pub fn number_literal(&self) -> Option<NumberLiteral> {
        todo![]
    }
    pub fn object_literal(&self) -> Option<ObjectLiteral> {
        todo![]
    }
    pub fn string_literal(&self) -> Option<StringLiteral> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl StringLiteral {
    pub fn string(&self) -> Option<SyntaxToken> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NumberLiteral {
    pub fn number(&self) -> Option<SyntaxToken> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl BooleanLiteral {
    pub fn false_keyword(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn true_keyword(&self) -> Option<SyntaxToken> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NullLiteral {
    pub fn null_keyword(&self) -> Option<SyntaxToken> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ArrayLiteral {
    pub fn literal_list(&self) -> Option<LiteralList> {
        todo![]
    }
    pub fn l_bracket(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn r_bracket(&self) -> Option<SyntaxToken> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ObjectLiteral {
    pub fn literal_field_list(&self) -> Option<LiteralFieldList> {
        todo![]
    }
    pub fn l_curly(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn r_curly(&self) -> Option<SyntaxToken> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl LiteralList {
    pub fn comma(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn literals(&self) -> std::vec::Vec<Literal> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl LiteralFieldList {
    pub fn comma(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn literal_fields(&self) -> std::vec::Vec<LiteralField> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl LiteralField {
    pub fn colon(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn key(&self) -> Option<Name> {
        todo![]
    }
    pub fn value(&self) -> Option<Literal> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeDecl {
    pub fn equal(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn field_list(&self) -> Option<FieldList> {
        todo![]
    }
    pub fn name(&self) -> Option<Name> {
        todo![]
    }
    pub fn type_keyword(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn l_curly(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn r_curly(&self) -> Option<SyntaxToken> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FieldList {
    pub fn comma(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn fields(&self) -> std::vec::Vec<Field> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeExpr {
    pub fn name(&self) -> Option<Name> {
        todo![]
    }
    pub fn type_array(&self) -> Option<TypeArray> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeArray {
    pub fn type_expr(&self) -> Option<TypeExpr> {
        todo![]
    }
    pub fn l_bracket(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn r_bracket(&self) -> Option<SyntaxToken> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Field {
    pub fn colon(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn decorator_attrs(&self) -> Option<DecoratorAttrs> {
        todo![]
    }
    pub fn macro_attrs(&self) -> Option<MacroAttrs> {
        todo![]
    }
    pub fn field_name(&self) -> Option<Name> {
        todo![]
    }
    pub fn field_type(&self) -> Option<TypeExpr> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MacroAttrs {
    pub fn macro_attrs(&self) -> std::vec::Vec<MacroAttr> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl DecoratorAttrs {
    pub fn decorator_attrs(&self) -> std::vec::Vec<DecoratorAttr> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MacroAttr {
    pub fn hash(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn l_parenthesis(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn r_parenthesis(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn literal_list(&self) -> Option<LiteralList> {
        todo![]
    }
    pub fn name(&self) -> Option<Name> {
        todo![]
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
        todo![]
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl DecoratorAttr {
    pub fn l_parenthesis(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn r_parenthesis(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn at(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn literal_list(&self) -> Option<LiteralList> {
        todo![]
    }
    pub fn name(&self) -> Option<Name> {
        todo![]
    }
}
