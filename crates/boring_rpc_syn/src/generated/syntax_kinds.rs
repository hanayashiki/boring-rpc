#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SyntaxKind {
    #[doc = r"token kind"]
    TypeKeyword,
    #[doc = r"token kind"]
    TrueKeyword,
    #[doc = r"token kind"]
    FalseKeyword,
    #[doc = r"token kind"]
    NullKeyword,
    #[doc = r"token kind"]
    ImportKeyword,
    #[doc = r"token kind"]
    FromKeyword,
    #[doc = r"token kind"]
    Ident,
    #[doc = r"token kind"]
    String,
    #[doc = r"token kind"]
    Number,
    #[doc = r"token kind"]
    Whitespace,
    #[doc = r"token kind"]
    Tab,
    #[doc = r"token kind"]
    Newline,
    #[doc = r"token kind"]
    EOF,
    #[doc = r"token kind"]
    Invalid,
    #[doc = r"token kind"]
    Equal,
    #[doc = r"token kind"]
    LCurly,
    #[doc = r"token kind"]
    RCurly,
    #[doc = r"token kind"]
    LBracket,
    #[doc = r"token kind"]
    RBracket,
    #[doc = r"token kind"]
    LParenthesis,
    #[doc = r"token kind"]
    RParenthesis,
    #[doc = r"token kind"]
    Comma,
    #[doc = r"token kind"]
    Hash,
    #[doc = r"token kind"]
    At,
    #[doc = r"token kind"]
    Colon,
    #[doc = r"token kind"]
    Star,
    #[doc = r"node kind"]
    Name,
    #[doc = r"node kind"]
    Literal,
    #[doc = r"node kind"]
    StringLiteral,
    #[doc = r"node kind"]
    NumberLiteral,
    #[doc = r"node kind"]
    BooleanLiteral,
    #[doc = r"node kind"]
    NullLiteral,
    #[doc = r"node kind"]
    ArrayLiteral,
    #[doc = r"node kind"]
    ObjectLiteral,
    #[doc = r"node kind"]
    LiteralList,
    #[doc = r"node kind"]
    LiteralFieldList,
    #[doc = r"node kind"]
    LiteralField,
    #[doc = r"node kind"]
    TypeDecl,
    #[doc = r"node kind"]
    FieldList,
    #[doc = r"node kind"]
    TypeExpr,
    #[doc = r"node kind"]
    TypeArray,
    #[doc = r"node kind"]
    Field,
    #[doc = r"node kind"]
    MacroAttrs,
    #[doc = r"node kind"]
    DecoratorAttrs,
    #[doc = r"node kind"]
    MacroAttr,
    #[doc = r"node kind"]
    DecoratorAttr,
    #[doc = r"node kind"]
    Module,
    #[doc = r"node kind"]
    StatementList,
    #[doc = r"node kind"]
    Statement,
    #[doc = r"node kind"]
    ImportDecl,
    #[doc = r"node kind"]
    ImportBody,
    #[doc = r"node kind"]
    ImportSource,
    #[doc = r"node kind"]
    ImportSpecifierList,
    #[doc = r"node kind"]
    ImportSpecifier,
}
impl SyntaxKind {
    pub fn to_ungram_name(&self) -> &'static str {
        match self {
            SyntaxKind::TypeKeyword => "type",
            SyntaxKind::TrueKeyword => "true",
            SyntaxKind::FalseKeyword => "false",
            SyntaxKind::NullKeyword => "null",
            SyntaxKind::ImportKeyword => "import",
            SyntaxKind::FromKeyword => "from",
            SyntaxKind::Ident => "#ident",
            SyntaxKind::String => "#string",
            SyntaxKind::Number => "#number",
            SyntaxKind::Whitespace => " ",
            SyntaxKind::Tab => "\t",
            SyntaxKind::Newline => "\n",
            SyntaxKind::EOF => "EOF",
            SyntaxKind::Invalid => "INVALID",
            SyntaxKind::Equal => "=",
            SyntaxKind::LCurly => "{",
            SyntaxKind::RCurly => "}",
            SyntaxKind::LBracket => "[",
            SyntaxKind::RBracket => "]",
            SyntaxKind::LParenthesis => "(",
            SyntaxKind::RParenthesis => ")",
            SyntaxKind::Comma => ",",
            SyntaxKind::Hash => "#",
            SyntaxKind::At => "@",
            SyntaxKind::Colon => ":",
            SyntaxKind::Star => "*",
            SyntaxKind::Name => "Name",
            SyntaxKind::Literal => "Literal",
            SyntaxKind::StringLiteral => "StringLiteral",
            SyntaxKind::NumberLiteral => "NumberLiteral",
            SyntaxKind::BooleanLiteral => "BooleanLiteral",
            SyntaxKind::NullLiteral => "NullLiteral",
            SyntaxKind::ArrayLiteral => "ArrayLiteral",
            SyntaxKind::ObjectLiteral => "ObjectLiteral",
            SyntaxKind::LiteralList => "LiteralList",
            SyntaxKind::LiteralFieldList => "LiteralFieldList",
            SyntaxKind::LiteralField => "LiteralField",
            SyntaxKind::TypeDecl => "TypeDecl",
            SyntaxKind::FieldList => "FieldList",
            SyntaxKind::TypeExpr => "TypeExpr",
            SyntaxKind::TypeArray => "TypeArray",
            SyntaxKind::Field => "Field",
            SyntaxKind::MacroAttrs => "MacroAttrs",
            SyntaxKind::DecoratorAttrs => "DecoratorAttrs",
            SyntaxKind::MacroAttr => "MacroAttr",
            SyntaxKind::DecoratorAttr => "DecoratorAttr",
            SyntaxKind::Module => "Module",
            SyntaxKind::StatementList => "StatementList",
            SyntaxKind::Statement => "Statement",
            SyntaxKind::ImportDecl => "ImportDecl",
            SyntaxKind::ImportBody => "ImportBody",
            SyntaxKind::ImportSource => "ImportSource",
            SyntaxKind::ImportSpecifierList => "ImportSpecifierList",
            SyntaxKind::ImportSpecifier => "ImportSpecifier",
        }
    }
}
