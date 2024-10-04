use boring_rpc_syn::{SyntaxKind, SyntaxToken};

struct Parser {
    tokens: Vec<SyntaxToken>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SyntaxToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.tokens[self.pos].kind() == kind {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    pub fn eat_keyword(&mut self) {}

    pub fn parse_type_decl(&self) {
        
    }
}