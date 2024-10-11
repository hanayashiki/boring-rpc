use boring_rpc_syn::{
    syn::{GreenNode, GreenNodeOrToken::*},
    syntax_error::SyntaxError,
    GreenToken, SyntaxKind, TextRange, TextSize,
};

use crate::lexed_str::LexedStr;

#[cfg(test)]
mod test_parser;

pub struct Parser {
    tokens: Vec<GreenToken>,
    pos: usize,
    errors: Vec<SyntaxError>,
}

impl Parser {
    pub fn new(tokens: Vec<GreenToken>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    pub fn of(s: &str) -> Self {
        Parser::new(LexedStr::new(s).to_tokens())
    }

    pub fn eat(&mut self, kind: SyntaxKind) -> Option<GreenToken> {
        let token = self.peek();
        if token.kind() == kind {
            let r = Some(token.clone());
            self.pos += 1;
            r
        } else {
            None
        }
    }

    fn peek(&self) -> &GreenToken {
        &self.tokens[self.pos]
    }

    fn range_of(&self, pos: usize) -> TextRange {
        let mut offset = TextSize::new(0);
        for i in 0..pos {
            offset += TextSize::of(self.tokens[i].value());
        }

        TextRange::at(offset, TextSize::of(self.peek().value()))
    }

    fn cur_range(&self) -> TextRange {
        self.range_of(self.pos)
    }

    fn eat_keyword(&mut self) -> Option<GreenToken> {
        let cur = self.peek();

        let kw = match (cur.kind(), cur.value()) {
            (SyntaxKind::Ident, "type") => Some(cur.with_kind(SyntaxKind::TypeKeyword)),
            (SyntaxKind::Ident, "true") => Some(cur.with_kind(SyntaxKind::TrueKeyword)),
            (SyntaxKind::Ident, "false") => Some(cur.with_kind(SyntaxKind::FalseKeyword)),
            _ => None,
        };

        if kw.is_some() {
            self.pos += 1;
        }

        kw
    }

    fn expect_token(&mut self, kind: SyntaxKind) -> Option<GreenToken> {
        let ident = self.eat(kind);

        ident.or_else(|| {
            self.errors.push(SyntaxError::new(
                format!(
                    "Expect '{}', got '{}'",
                    kind.to_ungram_name(),
                    self.peek().kind().to_ungram_name()
                ),
                self.cur_range(),
            ));
            None
        })
    }

    fn parse_type_decl(&mut self) -> GreenNode {
        let type_kw = self.eat_keyword().unwrap();
        assert!(type_kw.kind() == SyntaxKind::TypeKeyword);

        self.eat(SyntaxKind::Whitespace);

        let mut node = GreenNode::new(SyntaxKind::TypeDecl, vec![Token(type_kw)]);

        self.expect_token(SyntaxKind::Ident)
            .map(|ident| node.push(Token(ident)));

        self.eat(SyntaxKind::Whitespace);

        self.expect_token(SyntaxKind::Equal)
            .map(|ident| node.push(Token(ident)));

        self.eat(SyntaxKind::Whitespace);

        self.expect_token(SyntaxKind::LCurly)
            .map(|ident| node.push(Token(ident)));

        self.eat(SyntaxKind::Whitespace);

        node.push(Node(self.parse_field_list()));

        self.expect_token(SyntaxKind::RCurly)
            .map(|ident| node.push(Token(ident)));

        node
    }

    fn parse_field_list(&mut self) -> GreenNode {
        // TODO
        GreenNode::new(SyntaxKind::FieldList, Vec::new())
    }
}
