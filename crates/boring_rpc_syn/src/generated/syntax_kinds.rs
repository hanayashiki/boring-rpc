#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SyntaxKind {
    #[doc = r"token kind"]
    TypeKeyword,
    #[doc = r"token kind"]
    Ident,
    #[doc = r"token kind"]
    StringLiteral,
    #[doc = r"token kind"]
    NumberLiteral,
    #[doc = r"token kind"]
    Whitespace,
    #[doc = r"token kind"]
    Tab,
    #[doc = r"token kind"]
    Newline,
    #[doc = r"token kind"]
    EOF,
    #[doc = r"token kind"]
    Equal,
    #[doc = r"token kind"]
    LCurly,
    #[doc = r"token kind"]
    RCurly,
    #[doc = r"node kind"]
    Name,
    #[doc = r"node kind"]
    Literal,
    #[doc = r"node kind"]
    TypeDecl,
    #[doc = r"node kind"]
    FieldList,
    #[doc = r"node kind"]
    TypeExpr,
    #[doc = r"node kind"]
    Field,
    #[doc = r"node kind"]
    Macros,
    #[doc = r"node kind"]
    Decorators,
    #[doc = r"node kind"]
    Macro,
    #[doc = r"node kind"]
    LiteralList,
    #[doc = r"node kind"]
    Decorator,
}
