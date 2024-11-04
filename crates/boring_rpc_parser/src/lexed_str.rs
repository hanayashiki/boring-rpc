use boring_rpc_syn::{GreenToken, SyntaxKind};

pub struct LexedStr<'a> {
    str: &'a str,
    tokens: Vec<GreenToken>,
}

impl<'a> LexedStr<'a> {
    pub fn new(str: &'a str) -> Self {
        let mut iter = str.chars().peekable();
        let mut tokens = Vec::new();

        while let Some(c) = iter.peek().cloned() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    iter.next();
                    let mut value = format!("{}", c);
                    while let Some(c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$')) =
                        iter.peek().cloned()
                    {
                        value.push(c);
                        iter.next();
                    }
                    tokens.push(GreenToken::new(SyntaxKind::Ident, value));
                }
                '0'..='9' => {
                    // see https://www.json.org/img/number.png

                    let mut value = String::with_capacity(16);

                    while let Some(c) = iter.peek().cloned() {
                        // Integer part
                        match c {
                            '1'..='9' => {
                                value.push(c);
                                iter.next();

                                while let Some(c @ '0'..'9') = iter.peek().cloned() {
                                    value.push(c);
                                    iter.next();
                                }
                                break;
                            }
                            '0' => {
                                value.push(c);
                                iter.next();
                                break;
                            }
                            _ => break,
                        }
                    }

                    if let Some(c @ '.') = iter.peek().cloned() {
                        value.push(c);
                        iter.next();

                        while let Some(c @ '0'..='9') = iter.peek().cloned() {
                            value.push(c);
                            iter.next();
                        }
                        // However it is either digit or E here
                    }
                    // TODO: handle exp part
                    tokens.push(GreenToken::new(SyntaxKind::Number, value));
                }
                '\'' => {
                    iter.next();
                    let mut value = String::new();
                    while let Some(c) = iter.next() {
                        // handle esacpe
                        if c == '\\' {
                            match iter.next() {
                                Some('\'') => value.push('\''),
                                Some('\\') => value.push('\\'),
                                Some('n') => value.push('\n'),
                                Some('r') => value.push('\r'),
                                Some('t') => value.push('\t'),
                                Some('u') => {
                                    todo!();
                                }
                                Some(c) => {
                                    value.push(c);
                                },
                                None => {
                                    break;
                                }
                            }
                            continue;
                        }
                        if c == '\'' {
                            break;
                        }
                        value.push(c);
                    }
                    tokens.push(GreenToken::new(SyntaxKind::String, value));
                }
                ' ' | '\t' | '\n' | '\r' => {
                    let mut value = String::with_capacity(16);

                    while let Some(c @ (' ' | '\t' | '\n' | '\r')) = iter.peek().cloned() {
                        value.push(c);
                        iter.next();
                    }
                    tokens.push(GreenToken::new(SyntaxKind::Whitespace, value));
                }
                '=' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::Equal, "=".to_string()));
                }
                '{' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::LCurly, "{".to_string()));
                }
                '}' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::RCurly, "}".to_string()));
                }
                '[' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::LCurly, "[".to_string()));
                }
                ']' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::RCurly, "[".to_string()));
                }
                '(' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::LParenthesis, "(".to_string()));
                }
                ')' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::RParenthesis, ")".to_string()));
                }
                ',' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::Comma, ",".to_string()));
                }
                '#' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::Hash, "#".to_string()));
                }
                '@' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::At, "@".to_string()));
                }
                ':' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::Colon, ":".to_string()));
                }
                '*' => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::Star, "*".to_string()));
                }
                _ => {
                    iter.next();
                    tokens.push(GreenToken::new(SyntaxKind::Invalid, c.to_string()));
                }
            };
        }

        tokens.push(GreenToken::new(SyntaxKind::EOF, "".to_string()));

        Self { str, tokens }
    }

    pub fn to_tokens(self) -> Vec<GreenToken> {
        self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexed_str(input: &str, mut expected: Vec<GreenToken>) {
        let result = LexedStr::new(input).tokens;

        expected.push(GreenToken::new(SyntaxKind::EOF, "".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ident() {
        test_lexed_str(
            "abc",
            vec![GreenToken::new(SyntaxKind::Ident, "abc".to_string())],
        );

        test_lexed_str(
            "type",
            vec![GreenToken::new(SyntaxKind::Ident, "type".to_string())],
        );

        test_lexed_str(
            "yaju114514_$",
            vec![GreenToken::new(
                SyntaxKind::Ident,
                "yaju114514_$".to_string(),
            )],
        );
    }

    #[test]
    fn parse_number() {
        test_lexed_str(
            "12345",
            vec![GreenToken::new(SyntaxKind::Number, "12345".to_string())],
        );

        test_lexed_str(
            "912.3456789",
            vec![GreenToken::new(
                SyntaxKind::Number,
                "912.3456789".to_string(),
            )],
        );

        test_lexed_str(
            "0.11",
            vec![GreenToken::new(SyntaxKind::Number, "0.11".to_string())],
        );
    }

    #[test]
    fn parse_whitespace() {
        test_lexed_str(
            " \t",
            vec![GreenToken::new(SyntaxKind::Whitespace, " \t".to_string())],
        );

        test_lexed_str(
            "a b c",
            vec![
                GreenToken::new(SyntaxKind::Ident, "a".to_string()),
                GreenToken::new(SyntaxKind::Whitespace, " ".to_string()),
                GreenToken::new(SyntaxKind::Ident, "b".to_string()),
                GreenToken::new(SyntaxKind::Whitespace, " ".to_string()),
                GreenToken::new(SyntaxKind::Ident, "c".to_string()),
            ],
        );
    }
}
