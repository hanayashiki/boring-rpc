use std::collections::BTreeMap;

use ungrammar::{Grammar, Rule};

#[derive(Debug, Clone)]
pub enum NodeField {
    /// Terminals
    Token(String),
    /// Non-terminals
    Node { ty: String, many: bool },
}

pub struct RuleCollector<'a> {
    grammar: &'a Grammar,
    rule: &'a Rule,
    fields: BTreeMap<String, NodeField>,
    /// Are we collecting inside a [Rule::Rep]?
    many: bool,
}

/// Collects fields of a rule where concrete syntax is stored.
/// 
/// Fields are given default names by heuristics. Name collisions are not handled, and the last definition wins.
impl RuleCollector<'_> {
    pub fn new<'a>(grammar: &'a Grammar, rule: &'a Rule) -> RuleCollector<'a> {
        RuleCollector {
            grammar,
            rule,
            fields: BTreeMap::new(),
            many: false,
        }
    }

    pub fn many(&mut self) -> &mut Self {
        self.many = true;
        self
    }

    pub fn collect(&mut self) -> &BTreeMap<String, NodeField> {
        self.collect_rule(self.rule);
        &self.fields
    }

    fn collect_rule(&mut self, rule: &Rule) {
        match rule {
            Rule::Token(token) => {
                let token = &self.grammar[*token];

                self.fields
                    .insert(token.name.clone(), NodeField::Token(token.name.clone()));
            }
            Rule::Node(node) => {
                let node = &self.grammar[*node];
                let ty = node.name.clone();
                self.fields.insert(
                    node.name.clone(),
                    NodeField::Node {
                        ty,
                        many: self.many,
                    },
                );
            }
            Rule::Seq(seq) | Rule::Alt(seq) => {
                for rule in seq {
                    self.collect_rule(rule);
                }
            }
            Rule::Opt(rule) => {
                self.collect_rule(rule);
            }
            Rule::Rep(rule) => {
                let mut inner_collector = RuleCollector::new(self.grammar, rule);
                inner_collector.many();

                self.fields.extend(inner_collector.collect().clone());
            }
            Rule::Labeled { rule, label } => {
                if let Rule::Node(node) = **rule {
                    let node = &self.grammar[node];
                    let ty = node.name.clone();
                    self.fields.insert(
                        label.clone(),
                        NodeField::Node {
                            ty,
                            many: self.many,
                        },
                    );
                } else {
                    assert!(false, "Labeled rule must be a node");
                }
            }
        }
    }
}
