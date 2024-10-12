
use crate::syn::{AstNode, SyntaxNode, SyntaxToken};
use crate::SyntaxKind;

#[derive(Debug, Clone)]
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
impl Name {
    pub fn ident(&self) -> Option<SyntaxToken> {
        todo![]
    }
}

#[derive(Debug, Clone)]
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
impl Literal {
    pub fn array_literal(&self) -> Option<ArrayLiteral> {
        self.syntax().cast_child::<ArrayLiteral>()
    }
    pub fn boolean_literal(&self) -> Option<BooleanLiteral> {
        self.syntax().cast_child::<BooleanLiteral>()
    }
    pub fn null_literal(&self) -> Option<NullLiteral> {
        self.syntax().cast_child::<NullLiteral>()
    }
    pub fn number_literal(&self) -> Option<NumberLiteral> {
        self.syntax().cast_child::<NumberLiteral>()
    }
    pub fn object_literal(&self) -> Option<ObjectLiteral> {
        self.syntax().cast_child::<ObjectLiteral>()
    }
    pub fn string_literal(&self) -> Option<StringLiteral> {
        self.syntax().cast_child::<StringLiteral>()
    }
}

#[derive(Debug, Clone)]
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
impl StringLiteral {
    pub fn string(&self) -> Option<SyntaxToken> {
        todo![]
    }
}

#[derive(Debug, Clone)]
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
impl NumberLiteral {
    pub fn number(&self) -> Option<SyntaxToken> {
        todo![]
    }
}

#[derive(Debug, Clone)]
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
impl BooleanLiteral {
    pub fn false_keyword(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn true_keyword(&self) -> Option<SyntaxToken> {
        todo![]
    }
}

#[derive(Debug, Clone)]
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
impl NullLiteral {
    pub fn null_keyword(&self) -> Option<SyntaxToken> {
        todo![]
    }
}

#[derive(Debug, Clone)]
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
impl ArrayLiteral {
    pub fn literal_list(&self) -> Option<LiteralList> {
        self.syntax().cast_child::<LiteralList>()
    }
    pub fn l_bracket(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn r_bracket(&self) -> Option<SyntaxToken> {
        todo![]
    }
}

#[derive(Debug, Clone)]
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
impl ObjectLiteral {
    pub fn literal_field_list(&self) -> Option<LiteralFieldList> {
        self.syntax().cast_child::<LiteralFieldList>()
    }
    pub fn l_curly(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn r_curly(&self) -> Option<SyntaxToken> {
        todo![]
    }
}

#[derive(Debug, Clone)]
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
impl LiteralList {
    pub fn comma(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn literals(&self) -> Vec<Literal> {
        self.syntax().cast_children::<Literal>()
    }
}

#[derive(Debug, Clone)]
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
impl LiteralFieldList {
    pub fn comma(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn literal_fields(&self) -> Vec<LiteralField> {
        self.syntax().cast_children::<LiteralField>()
    }
}

#[derive(Debug, Clone)]
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
impl LiteralField {
    pub fn colon(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn key(&self) -> Option<Name> {
        self.syntax().cast_child::<Name>()
    }
    pub fn value(&self) -> Option<Literal> {
        self.syntax().cast_child::<Literal>()
    }
}

#[derive(Debug, Clone)]
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
impl TypeDecl {
    pub fn equal(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn field_list(&self) -> Option<FieldList> {
        self.syntax().cast_child::<FieldList>()
    }
    pub fn name(&self) -> Option<Name> {
        self.syntax().cast_child::<Name>()
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

#[derive(Debug, Clone)]
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
impl FieldList {
    pub fn comma(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn fields(&self) -> Vec<Field> {
        self.syntax().cast_children::<Field>()
    }
}

#[derive(Debug, Clone)]
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
impl TypeExpr {
    pub fn name(&self) -> Option<Name> {
        self.syntax().cast_child::<Name>()
    }
    pub fn type_array(&self) -> Option<TypeArray> {
        self.syntax().cast_child::<TypeArray>()
    }
}

#[derive(Debug, Clone)]
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
impl TypeArray {
    pub fn type_expr(&self) -> Option<TypeExpr> {
        self.syntax().cast_child::<TypeExpr>()
    }
    pub fn l_bracket(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn r_bracket(&self) -> Option<SyntaxToken> {
        todo![]
    }
}

#[derive(Debug, Clone)]
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
impl Field {
    pub fn colon(&self) -> Option<SyntaxToken> {
        todo![]
    }
    pub fn decorator_attrs(&self) -> Option<DecoratorAttrs> {
        self.syntax().cast_child::<DecoratorAttrs>()
    }
    pub fn macro_attrs(&self) -> Option<MacroAttrs> {
        self.syntax().cast_child::<MacroAttrs>()
    }
    pub fn field_name(&self) -> Option<Name> {
        self.syntax().cast_child::<Name>()
    }
    pub fn field_type(&self) -> Option<TypeExpr> {
        self.syntax().cast_child::<TypeExpr>()
    }
}

#[derive(Debug, Clone)]
pub struct MacroAttrs {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for MacroAttrs {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::MacroAttrs == kind
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
impl MacroAttrs {
    pub fn macro_attrs(&self) -> Vec<MacroAttr> {
        self.syntax().cast_children::<MacroAttr>()
    }
}

#[derive(Debug, Clone)]
pub struct DecoratorAttrs {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DecoratorAttrs {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::DecoratorAttrs == kind
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
impl DecoratorAttrs {
    pub fn decorator_attrs(&self) -> Vec<DecoratorAttr> {
        self.syntax().cast_children::<DecoratorAttr>()
    }
}

#[derive(Debug, Clone)]
pub struct MacroAttr {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for MacroAttr {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::MacroAttr == kind
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
        self.syntax().cast_child::<LiteralList>()
    }
    pub fn name(&self) -> Option<Name> {
        self.syntax().cast_child::<Name>()
    }
}

#[derive(Debug, Clone)]
pub struct DecoratorAttr {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DecoratorAttr {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::DecoratorAttr == kind
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
        self.syntax().cast_child::<LiteralList>()
    }
    pub fn name(&self) -> Option<Name> {
        self.syntax().cast_child::<Name>()
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Module {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Module == kind
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
impl Module {
    pub fn type_decl_list(&self) -> Option<TypeDeclList> {
        self.syntax().cast_child::<TypeDeclList>()
    }
}

#[derive(Debug, Clone)]
pub struct TypeDeclList {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for TypeDeclList {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::TypeDeclList == kind
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
impl TypeDeclList {
    pub fn type_decls(&self) -> Vec<TypeDecl> {
        self.syntax().cast_children::<TypeDecl>()
    }
}
