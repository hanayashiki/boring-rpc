
use crate::syn::{AstNode, SyntaxNode, SyntaxToken};
use crate::{tokens::*, SyntaxKind};

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
    pub fn ident(&self) -> Option<Ident> {
        self.syntax().cast_token::<Ident>()
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
    pub fn string(&self) -> Option<String> {
        self.syntax().cast_token::<String>()
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
    pub fn number(&self) -> Option<Number> {
        self.syntax().cast_token::<Number>()
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
    pub fn false_keyword(&self) -> Option<FalseKeyword> {
        self.syntax().cast_token::<FalseKeyword>()
    }
    pub fn true_keyword(&self) -> Option<TrueKeyword> {
        self.syntax().cast_token::<TrueKeyword>()
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
    pub fn null_keyword(&self) -> Option<NullKeyword> {
        self.syntax().cast_token::<NullKeyword>()
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
    pub fn l_bracket(&self) -> Option<LBracket> {
        self.syntax().cast_token::<LBracket>()
    }
    pub fn r_bracket(&self) -> Option<RBracket> {
        self.syntax().cast_token::<RBracket>()
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
    pub fn l_curly(&self) -> Option<LCurly> {
        self.syntax().cast_token::<LCurly>()
    }
    pub fn r_curly(&self) -> Option<RCurly> {
        self.syntax().cast_token::<RCurly>()
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
    pub fn comma(&self) -> Option<Comma> {
        self.syntax().cast_token::<Comma>()
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
    pub fn comma(&self) -> Option<Comma> {
        self.syntax().cast_token::<Comma>()
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
    pub fn colon(&self) -> Option<Colon> {
        self.syntax().cast_token::<Colon>()
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
    pub fn equal(&self) -> Option<Equal> {
        self.syntax().cast_token::<Equal>()
    }
    pub fn field_list(&self) -> Option<FieldList> {
        self.syntax().cast_child::<FieldList>()
    }
    pub fn name(&self) -> Option<Name> {
        self.syntax().cast_child::<Name>()
    }
    pub fn type_keyword(&self) -> Option<TypeKeyword> {
        self.syntax().cast_token::<TypeKeyword>()
    }
    pub fn l_curly(&self) -> Option<LCurly> {
        self.syntax().cast_token::<LCurly>()
    }
    pub fn r_curly(&self) -> Option<RCurly> {
        self.syntax().cast_token::<RCurly>()
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
    pub fn comma(&self) -> Option<Comma> {
        self.syntax().cast_token::<Comma>()
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
    pub fn l_bracket(&self) -> Option<LBracket> {
        self.syntax().cast_token::<LBracket>()
    }
    pub fn r_bracket(&self) -> Option<RBracket> {
        self.syntax().cast_token::<RBracket>()
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
    pub fn colon(&self) -> Option<Colon> {
        self.syntax().cast_token::<Colon>()
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
    pub fn hash(&self) -> Option<Hash> {
        self.syntax().cast_token::<Hash>()
    }
    pub fn l_parenthesis(&self) -> Option<LParenthesis> {
        self.syntax().cast_token::<LParenthesis>()
    }
    pub fn r_parenthesis(&self) -> Option<RParenthesis> {
        self.syntax().cast_token::<RParenthesis>()
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
    pub fn l_parenthesis(&self) -> Option<LParenthesis> {
        self.syntax().cast_token::<LParenthesis>()
    }
    pub fn r_parenthesis(&self) -> Option<RParenthesis> {
        self.syntax().cast_token::<RParenthesis>()
    }
    pub fn at(&self) -> Option<At> {
        self.syntax().cast_token::<At>()
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
    pub fn statement_list(&self) -> Option<StatementList> {
        self.syntax().cast_child::<StatementList>()
    }
}

#[derive(Debug, Clone)]
pub struct StatementList {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for StatementList {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::StatementList == kind
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
impl StatementList {
    pub fn statements(&self) -> Vec<Statement> {
        self.syntax().cast_children::<Statement>()
    }
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Statement {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::Statement == kind
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
impl Statement {
    pub fn import_decl(&self) -> Option<ImportDecl> {
        self.syntax().cast_child::<ImportDecl>()
    }
    pub fn service_decl(&self) -> Option<ServiceDecl> {
        self.syntax().cast_child::<ServiceDecl>()
    }
    pub fn type_decl(&self) -> Option<TypeDecl> {
        self.syntax().cast_child::<TypeDecl>()
    }
}

#[derive(Debug, Clone)]
pub struct ImportDecl {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ImportDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ImportDecl == kind
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
impl ImportDecl {
    pub fn import_body(&self) -> Option<ImportBody> {
        self.syntax().cast_child::<ImportBody>()
    }
    pub fn import_source(&self) -> Option<ImportSource> {
        self.syntax().cast_child::<ImportSource>()
    }
    pub fn from_keyword(&self) -> Option<FromKeyword> {
        self.syntax().cast_token::<FromKeyword>()
    }
    pub fn import_keyword(&self) -> Option<ImportKeyword> {
        self.syntax().cast_token::<ImportKeyword>()
    }
}

#[derive(Debug, Clone)]
pub struct ServiceDecl {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ServiceDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ServiceDecl == kind
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
impl ServiceDecl {
    pub fn name(&self) -> Option<Name> {
        self.syntax().cast_child::<Name>()
    }
    pub fn service_method_list(&self) -> Option<ServiceMethodList> {
        self.syntax().cast_child::<ServiceMethodList>()
    }
    pub fn service_keyword(&self) -> Option<ServiceKeyword> {
        self.syntax().cast_token::<ServiceKeyword>()
    }
    pub fn l_curly(&self) -> Option<LCurly> {
        self.syntax().cast_token::<LCurly>()
    }
    pub fn r_curly(&self) -> Option<RCurly> {
        self.syntax().cast_token::<RCurly>()
    }
}

#[derive(Debug, Clone)]
pub struct ImportBody {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ImportBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ImportBody == kind
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
impl ImportBody {
    pub fn star(&self) -> Option<Star> {
        self.syntax().cast_token::<Star>()
    }
    pub fn import_specifier_list(&self) -> Option<ImportSpecifierList> {
        self.syntax().cast_child::<ImportSpecifierList>()
    }
    pub fn l_curly(&self) -> Option<LCurly> {
        self.syntax().cast_token::<LCurly>()
    }
    pub fn r_curly(&self) -> Option<RCurly> {
        self.syntax().cast_token::<RCurly>()
    }
}

#[derive(Debug, Clone)]
pub struct ImportSource {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ImportSource {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ImportSource == kind
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
impl ImportSource {
    pub fn string(&self) -> Option<String> {
        self.syntax().cast_token::<String>()
    }
}

#[derive(Debug, Clone)]
pub struct ImportSpecifierList {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ImportSpecifierList {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ImportSpecifierList == kind
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
impl ImportSpecifierList {
    pub fn comma(&self) -> Option<Comma> {
        self.syntax().cast_token::<Comma>()
    }
    pub fn import_specifiers(&self) -> Vec<ImportSpecifier> {
        self.syntax().cast_children::<ImportSpecifier>()
    }
}

#[derive(Debug, Clone)]
pub struct ImportSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ImportSpecifier {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ImportSpecifier == kind
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
impl ImportSpecifier {
    pub fn ident(&self) -> Option<Ident> {
        self.syntax().cast_token::<Ident>()
    }
}

#[derive(Debug, Clone)]
pub struct ServiceMethodList {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ServiceMethodList {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ServiceMethodList == kind
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
impl ServiceMethodList {
    pub fn comma(&self) -> Option<Comma> {
        self.syntax().cast_token::<Comma>()
    }
    pub fn service_methods(&self) -> Vec<ServiceMethod> {
        self.syntax().cast_children::<ServiceMethod>()
    }
}

#[derive(Debug, Clone)]
pub struct ServiceMethod {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ServiceMethod {
    fn can_cast(kind: SyntaxKind) -> bool {
        SyntaxKind::ServiceMethod == kind
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
impl ServiceMethod {
    pub fn l_parenthesis(&self) -> Option<LParenthesis> {
        self.syntax().cast_token::<LParenthesis>()
    }
    pub fn r_parenthesis(&self) -> Option<RParenthesis> {
        self.syntax().cast_token::<RParenthesis>()
    }
    pub fn colon(&self) -> Option<Colon> {
        self.syntax().cast_token::<Colon>()
    }
    pub fn decorator_attrs(&self) -> Option<DecoratorAttrs> {
        self.syntax().cast_child::<DecoratorAttrs>()
    }
    pub fn macro_attrs(&self) -> Option<MacroAttrs> {
        self.syntax().cast_child::<MacroAttrs>()
    }
    pub fn method_name(&self) -> Option<Name> {
        self.syntax().cast_child::<Name>()
    }
    pub fn method_return(&self) -> Option<TypeExpr> {
        self.syntax().cast_child::<TypeExpr>()
    }
    pub fn parameters(&self) -> Option<FieldList> {
        self.syntax().cast_child::<FieldList>()
    }
}
