use quote::{format_ident, quote};
use std::{
    fs::{self, create_dir_all, read_to_string},
    path::Path,
};
use xshell::{cmd, Shell};
use boring_rpc_common::to_lower_snake_case;

use ungrammar::Grammar;

use crate::codegen::{
    constants::TOKEN_DEFS,
    rule_collector::{NodeField, RuleCollector},
};

fn generate_syntax_kind_rs(grammar: &Grammar) -> String {
    let token_kinds = TOKEN_DEFS.iter().map(|(_, n)| format_ident!("{}", n));

    let node_kinds = grammar
        .iter()
        .map(|node| format_ident!("{}", &grammar[node].name));

    let token_to_ungram_name = TOKEN_DEFS.iter().map(|(ungram_name, struct_name)| {
        let struct_name = format_ident!("{}", struct_name);

        quote! {
            SyntaxKind::#struct_name => #ungram_name,
        }
    });

    let node_to_ungram_name = grammar.iter().map(|node| {
        let struct_name = format_ident!("{}", &grammar[node].name);
        let ungram_name = &grammar[node].name;

        quote! {
            SyntaxKind::#struct_name => #ungram_name,
        }
    });

    let ast = quote! {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum SyntaxKind {

            #(
                ///token kind
                #token_kinds,)*

            #(
                ///node kind
                #node_kinds,
            )*
        }

        impl SyntaxKind {
            pub fn to_ungram_name(&self) -> &'static str {
                match self {
                    #(#token_to_ungram_name)*
                    #(#node_to_ungram_name)*
                }
            }
        }
    };

    format!("{}", ast)
}

fn generate_tokens_rs() -> String {
    let structs = TOKEN_DEFS.iter().map(|(_, struct_name)| {
        let struct_name = format_ident!("{}", struct_name);

        format!(
            "\n\n{}",
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #struct_name {
                    pub(crate) syntax: SyntaxToken,
                }

                impl AstToken for #struct_name {
                    fn can_cast(kind: SyntaxKind) -> bool { SyntaxKind::#struct_name == kind }
                    fn cast(syntax: SyntaxToken) -> Option<Self> {
                        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                    }
                    fn syntax(&self) -> &SyntaxToken { &self.syntax }
                }
            }
        )
    });

    let header = "
        use crate::syn::{AstToken, SyntaxToken};
        use crate::{SyntaxKind};
    ";

    format!("{}\n{}", header, structs.collect::<String>())
}

fn generate_nodes_rs(grammar: &Grammar) -> String {
    let structs = grammar.iter().map(|node| {
        let node = &grammar[node];
        let struct_name = format_ident!("{}", node.name);

        let mut collector = RuleCollector::new(grammar, &node.rule);
        let fields = collector.collect();

        let field_methods = fields.iter().map(|(name, field)| match field {
            NodeField::Token(token_ungram_name) => {
                let struct_name = TOKEN_DEFS
                    .iter()
                    .find(|(ungram_name, _)| ungram_name == token_ungram_name)
                    .expect(format!("{} is not a valid token", token_ungram_name).as_str())
                    .1;
                let name = format_ident!("{}", to_lower_snake_case(struct_name));
                let ty = format_ident!("{}", struct_name);

                quote! {
                    pub fn #name(&self) -> Option<#ty> {
                        self.syntax().cast_token::<#ty>()
                    }
                }
            }
            NodeField::Node { many, ty } => {
                let name = format_ident!(
                    "{}{}",
                    to_lower_snake_case(name),
                    match many {
                        true => "s",
                        false => "",
                    }
                );
                let ty = format_ident!("{}", ty);

                if *many {
                    quote! {
                        pub fn #name(&self) -> Vec<#ty> {
                            self.syntax().cast_children::<#ty>()
                        }
                    }
                } else {
                    quote! {
                        pub fn #name(&self) -> Option<#ty> {
                            self.syntax().cast_child::<#ty>()
                        }
                    }
                }
            }
        });

        format!(
            "\n\n{}",
            quote! {
                #[derive(Debug, Clone)]
                pub struct #struct_name {
                    pub(crate) syntax: SyntaxNode,
                }

                impl AstNode for #struct_name {
                    fn can_cast(kind: SyntaxKind) -> bool { SyntaxKind::#struct_name == kind }
                    fn cast(syntax: SyntaxNode) -> Option<Self> {
                        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                    }
                    fn syntax(&self) -> &SyntaxNode { &self.syntax }
                }

                impl #struct_name {
                    #(#field_methods)*
                }
            },
        )
    });

    let header = "
        use crate::syn::{SyntaxToken, SyntaxNode, AstNode};
        use crate::{SyntaxKind, tokens::*};
    ";

    format!("{}\n{}", header, structs.collect::<String>())
}

pub fn generate_syn() {
    let grammar: Grammar = read_to_string("./boring_rpc.ungram")
        .unwrap()
        .parse()
        .unwrap();

    let root = Path::new("crates/boring_rpc_syn/src/generated/");

    write_pretty_code(
        &root.join("syntax_kinds.rs"),
        &generate_syntax_kind_rs(&grammar),
    );
    write_pretty_code(&root.join("tokens.rs"), &generate_tokens_rs());
    write_pretty_code(&root.join("nodes.rs"), &generate_nodes_rs(&grammar));
}

fn write_pretty_code(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        let _ = create_dir_all(parent);
    }
    fs::write(path, contents).unwrap();

    let sh = Shell::new().unwrap();
    cmd!(sh, "rustup run stable rustfmt {path}").run().unwrap();
}
