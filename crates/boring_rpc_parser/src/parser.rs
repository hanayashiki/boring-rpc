use boring_rpc_syn::{
    GreenNode,
    GreenNodeOrToken::{self, *},
    GreenToken, SyntaxError, SyntaxKind, TextRange, TextSize,
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

    fn peek(&self) -> &GreenToken {
        &self.tokens[self.pos]
    }

    /// If the peeked token matches `kind`, consume and return it. Otherwise, return None.
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

    /// Peek the next token and return it if it matches one of the keyword
    /// This allows treating keywords only when we need to, allowing the freedom of naming ids.
    fn peek_keyword(&self) -> Option<SyntaxKind> {
        match self.peek().kind() {
            SyntaxKind::Ident => match self.peek().value() {
                "type" => Some(SyntaxKind::TypeKeyword),
                "import" => Some(SyntaxKind::ImportKeyword),
                "true" => Some(SyntaxKind::TrueKeyword),
                "false" => Some(SyntaxKind::FalseKeyword),
                "from" => Some(SyntaxKind::FromKeyword),
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

    fn expect_keyword(&mut self, kw_kind: SyntaxKind) -> Option<GreenToken> {
        let ident = self.peek_keyword();

        match ident {
            Some(kind) if kind == kw_kind => self.eat_keyword(),
            _ => {
                self.errors.push(SyntaxError::new(
                    format!(
                        "Expect keyword '{}', got '{}'",
                        kw_kind.to_ungram_name(),
                        self.peek().kind().to_ungram_name()
                    ),
                    self.cur_range(),
                ));
                None
            }
        }
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

    fn expect_keyword_push(&mut self, node: &mut GreenNode, kw_kind: SyntaxKind) -> bool {
        match self.expect_keyword(kw_kind) {
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
            self.eat_push(&mut node, SyntaxKind::Whitespace);

            match self.peek_keyword() {
                Some(SyntaxKind::TypeKeyword | SyntaxKind::ImportKeyword) => {
                    node.push(Node(self.parse_statement_list()));
                }
                _ => match self.peek().kind() {
                    SyntaxKind::EOF => break,
                    kind => todo!(
                        "Unexpected token when parsing module: {:?} at {:?}",
                        kind.to_ungram_name(),
                        self.cur_range(),
                    ),
                },
            }
        }

        node
    }

    fn parse_statement_list(&mut self) -> GreenNode {
        assert!(matches!(
            self.peek_keyword(),
            Some(SyntaxKind::TypeKeyword | SyntaxKind::ImportKeyword)
        ));

        let mut statement_list_node = GreenNode::new(SyntaxKind::StatementList, Vec::new());
        loop {
            match self.peek_keyword() {
                Some(SyntaxKind::TypeKeyword | SyntaxKind::ImportKeyword) => {
                    statement_list_node.push(GreenNodeOrToken::Node(self.parse_statement()));
                }
                _ => break,
            }

            self.eat_push(&mut statement_list_node, SyntaxKind::Whitespace);
        }

        statement_list_node
    }

    fn parse_statement(&mut self) -> GreenNode {
        assert!(matches!(
            self.peek_keyword(),
            Some(SyntaxKind::TypeKeyword | SyntaxKind::ImportKeyword)
        ));

        let statement_node = GreenNode::new(
            SyntaxKind::Statement,
            vec![GreenNodeOrToken::Node(match self.peek_keyword() {
                Some(SyntaxKind::TypeKeyword) => self.parse_type_decl(),
                Some(SyntaxKind::ImportKeyword) => self.parse_import_decl(),
                _ => unreachable!(),
            })],
        );
        statement_node
    }

    fn parse_import_decl(&mut self) -> GreenNode {
        let type_kw = self.eat_keyword().unwrap();
        assert!(type_kw.kind() == SyntaxKind::ImportKeyword);

        let mut node = GreenNode::new(SyntaxKind::ImportDecl, vec![Token(type_kw)]);

        {
            let node = &mut node;

            self.eat_push(node, SyntaxKind::Whitespace);

            if let SyntaxKind::Star | SyntaxKind::LCurly = self.peek().kind() {
                node.push(GreenNodeOrToken::Node(self.parse_import_body()));
            } else {
                self.errors.push(SyntaxError::new(
                    format!("Expect `*` or `{{ ... }}` for the import specifier.",),
                    self.cur_range(),
                ));
            }

            self.eat_push(node, SyntaxKind::Whitespace);

            self.expect_keyword_push(node, SyntaxKind::FromKeyword);

            self.eat_push(node, SyntaxKind::Whitespace);

            if self.peek().kind() == SyntaxKind::String {
                node.push(GreenNodeOrToken::Node(self.parse_import_source()));
            }
        }

        node
    }

    fn parse_import_body(&mut self) -> GreenNode {
        assert!(matches!(
            self.peek().kind(),
            SyntaxKind::Star | SyntaxKind::LCurly
        ));

        let mut node = GreenNode::new(SyntaxKind::ImportBody, Vec::new());
        {
            let node = &mut node;

            if self.eat_push(node, SyntaxKind::Star) {
                // Done
            } else {
                self.expect_push(node, SyntaxKind::LCurly);
                self.eat_push(node, SyntaxKind::Whitespace);

                if self.peek().kind() == SyntaxKind::Ident {
                    node.push(GreenNodeOrToken::Node(self.parse_import_specifier_list()));
                }

                self.eat_push(node, SyntaxKind::Whitespace);
                self.expect_push(node, SyntaxKind::RCurly);
            }
        }

        node
    }

    fn parse_import_specifier_list(&mut self) -> GreenNode {
        assert_eq!(self.peek().kind(), SyntaxKind::Ident);

        let mut node = GreenNode::new(SyntaxKind::ImportSpecifierList, Vec::new());
        {
            let node = &mut node;

            node.push(Node(self.parse_import_specifier()));
            self.eat_push(node, SyntaxKind::Whitespace);

            loop {
                if self.peek().kind() == SyntaxKind::Comma {
                    self.eat_push(node, SyntaxKind::Comma);
                    self.eat_push(node, SyntaxKind::Whitespace);

                    if self.peek().kind() == SyntaxKind::Ident {
                        node.push(Node(self.parse_import_specifier()));
                        self.eat_push(node, SyntaxKind::Whitespace);
                    } else if self.peek().kind() == SyntaxKind::RCurly {
                        break;
                    } else {
                        self.errors.push(SyntaxError::new(
                            format!("Expect an identifier or `}}` for the import specifier list.",),
                            self.cur_range(),
                        ));
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        node
    }

    fn parse_import_specifier(&mut self) -> GreenNode {
        self.parse_single_token_node(SyntaxKind::ImportSpecifier, SyntaxKind::Ident)
    }

    fn parse_import_source(&mut self) -> GreenNode {
        self.parse_single_token_node(SyntaxKind::ImportSource, SyntaxKind::String)
    }

    fn parse_type_decl(&mut self) -> GreenNode {
        let type_kw = self.eat_keyword().unwrap();
        assert!(type_kw.kind() == SyntaxKind::TypeKeyword);

        let mut node = GreenNode::new(SyntaxKind::TypeDecl, vec![Token(type_kw)]);

        {
            let node = &mut node;

            self.eat_push(node, SyntaxKind::Whitespace);

            if self.peek().kind() == SyntaxKind::Ident {
                node.push(Node(self.parse_name()));
            }

            self.eat_push(node, SyntaxKind::Whitespace);
            self.expect_push(node, SyntaxKind::Equal);
            self.eat_push(node, SyntaxKind::Whitespace);
            self.expect_push(node, SyntaxKind::LCurly);
            self.eat_push(node, SyntaxKind::Whitespace);

            if let SyntaxKind::Hash | SyntaxKind::At | SyntaxKind::Ident = self.peek().kind() {
                node.push(Node(self.parse_field_list()))
            }

            self.expect_push(node, SyntaxKind::RCurly);
        }

        node
    }

    fn parse_field_list(&mut self) -> GreenNode {
        let mut node = GreenNode::new(SyntaxKind::FieldList, Vec::new());
        {
            let node = &mut node;

            // TODO: macros and decorators
            loop {
                match self.peek().kind() {
                    SyntaxKind::Ident => {
                        node.push(Node(self.parse_field()));

                        self.eat_push(node, SyntaxKind::Whitespace);
                        self.eat_push(node, SyntaxKind::Comma);
                        self.eat_push(node, SyntaxKind::Whitespace);
                    }
                    _ => break,
                }
            }
        }

        node
    }

    fn parse_field(&mut self) -> GreenNode {
        assert!(self.peek().kind() == SyntaxKind::Ident);

        let mut node = GreenNode::new(SyntaxKind::Field, vec![]);
        {
            let node = &mut node;

            // field_name
            node.push(Node(self.parse_name()));

            self.eat_push(node, SyntaxKind::Whitespace);
            self.expect_push(node, SyntaxKind::Colon);
            // field_type
            self.eat_push(node, SyntaxKind::Whitespace);
            node.push(Node(self.parse_type_expr()));
        }

        node
    }

    fn parse_single_token_node(
        &mut self,
        node_kind: SyntaxKind,
        token_kind: SyntaxKind,
    ) -> GreenNode {
        let token = self.eat(token_kind).unwrap();
        GreenNode::new(node_kind, vec![Token(token)])
    }

    fn parse_name(&mut self) -> GreenNode {
        self.parse_single_token_node(SyntaxKind::Name, SyntaxKind::Ident)
    }

    fn parse_type_expr(&mut self) -> GreenNode {
        // FIXME: other type exprs
        GreenNode::new(SyntaxKind::TypeExpr, vec![Node(self.parse_name())])
    }
}
