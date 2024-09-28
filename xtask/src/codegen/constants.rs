// (ungram_name, struct_name)
pub static TOKEN_DEFS: &'static [(&str, &str)] = &[
    // Keywords
    ("type", "TypeKeyword"),
    ("true", "TrueKeyword"),
    ("false", "FalseKeyword"),
    ("null", "NullKeyword"),
    // Literals
    ("#ident", "Ident"),
    ("#string", "String"),
    ("#number", "Number"),
    // Trivia
    (" ", "Whitespace"),
    ("\t", "Tab"),
    ("\n", "Newline"),
    ("EOF", "EOF"),
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
];
