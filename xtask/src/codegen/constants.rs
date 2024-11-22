// (ungram_name, struct_name)
pub static TOKEN_DEFS: &'static [(&str, &str)] = &[
    // Keywords
    ("type", "TypeKeyword"),
    ("true", "TrueKeyword"),
    ("false", "FalseKeyword"),
    ("null", "NullKeyword"),
    ("import", "ImportKeyword"),
    ("from", "FromKeyword"),
    ("service", "ServiceKeyword"),
    // Literals
    ("#ident", "Ident"),
    ("#string", "String"),
    ("#number", "Number"),
    // Trivia
    (" ", "Whitespace"),
    ("\t", "Tab"),
    ("\n", "Newline"),
    ("EOF", "EOF"),
    ("INVALID", "Invalid"),
    // Puncts
    ("=", "Equal"),
    ("{", "LCurly"),
    ("}", "RCurly"),
    ("[", "LBracket"),
    ("]", "RBracket"),
    ("(", "LParenthesis"),
    (")", "RParenthesis"),
    (",", "Comma"),
    ("#", "Hash"),
    ("@", "At"),
    (":", "Colon"),
    ("*", "Star"),
];
