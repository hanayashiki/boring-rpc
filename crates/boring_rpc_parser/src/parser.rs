use boring_rpc_syn::{
    tokens::Whitespace, GreenNode, GreenNodeOrToken::*, GreenToken, SyntaxError, SyntaxKind,
    TextRange, TextSize,
};

use crate::lexed_str::LexedStr;

#[cfg(test)]
mod test_parser;

#[derive(Default, Debug)]
pub struct Parser {
    tokens: Vec<GreenToken>,
    pos: usize,
    errors: Vec<SyntaxError>,
}

impl Parser {
    pub fn of(s: &str) -> Self {
        let mut p = Parser::default();
        p.tokens = LexedStr::new(s).to_tokens();
        return p;
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

    pub fn eat_push(&mut self, node: &mut GreenNode, kind: SyntaxKind) -> bool {
        match self.eat(kind) {
            Some(token) => {
                node.push(Token(token));
                true
            }
            None => false,
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

    fn peek_keyword(&self) -> Option<SyntaxKind> {
        match self.peek().kind() {
            SyntaxKind::Ident => match self.peek().value() {
                "type" => Some(SyntaxKind::TypeKeyword),
                "true" => Some(SyntaxKind::TrueKeyword),
                "false" => Some(SyntaxKind::FalseKeyword),
                _ => None,
            },
            _ => None,
        }
    }

    fn eat_keyword(&mut self) -> Option<GreenToken> {
        let kw = self.peek_keyword().map(|kind| self.peek().with_kind(kind));

        if kw.is_some() {
            self.pos += 1;
        }

        kw
    }

    fn expect(&mut self, kind: SyntaxKind) -> Option<GreenToken> {
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

    fn expect_push(&mut self, node: &mut GreenNode, kind: SyntaxKind) -> bool {
        match self.expect(kind) {
            Some(token) => {
                node.push(Token(token));
                true
            }
            None => false,
        }
    }

    pub fn parse_module(&mut self) -> GreenNode {
        let mut node = GreenNode::new(SyntaxKind::Module, Vec::new());
        loop {
            match self.peek_keyword() {
                Some(SyntaxKind::TypeKeyword) => {
                    node.push(Node(self.parse_type_decl()));
                }
                _ => match self.peek().kind() {
                    SyntaxKind::EOF => break,
                    _ => todo!("error handling"),
                },
            }

            self.eat(SyntaxKind::Whitespace);
        }

        node
    }

    fn parse_type_decl(&mut self) -> GreenNode {
        let type_kw = self.eat_keyword().unwrap();
        assert!(type_kw.kind() == SyntaxKind::TypeKeyword);

        self.eat(SyntaxKind::Whitespace);

        let mut node = GreenNode::new(SyntaxKind::TypeDecl, vec![Token(type_kw)]);
        {
            let node = &mut node;

            self.expect_push(node, SyntaxKind::Ident);
            self.eat_push(node, SyntaxKind::Whitespace);
            self.expect_push(node, SyntaxKind::Equal);
            self.eat_push(node, SyntaxKind::Whitespace);
            self.expect_push(node, SyntaxKind::LCurly);
            self.eat_push(node, SyntaxKind::Whitespace);

            node.push(Node(self.parse_field_list()));

            self.expect_push(node, SyntaxKind::RCurly);
        }

        node
    }

    fn parse_field_list(&mut self) -> GreenNode {
        // TODO
        GreenNode::new(SyntaxKind::FieldList, Vec::new())
    }
}
