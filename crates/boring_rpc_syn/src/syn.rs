use std::rc::{Rc, Weak};

use text_size::TextRange;

use crate::{GreenNode, GreenNodeOrToken, SyntaxKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyntaxNodeId {
    pub kind: SyntaxKind,
    pub offset: u32,
}

pub trait AstToken {
    fn can_cast(kind: SyntaxKind) -> bool;
    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;
    fn syntax(&self) -> &SyntaxToken;
}

pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool;
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;
    fn syntax(&self) -> &SyntaxNode;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken {
    offset: u32,
    kind: SyntaxKind,
    value: String,
}

impl SyntaxToken {
    pub fn new(offset: u32, kind: SyntaxKind, value: String) -> Self {
        Self {
            offset,
            kind,
            value,
        }
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxNode(Rc<SyntaxNodeInner>);

impl SyntaxNode {
    fn green_node(&self) -> &GreenNode {
        unsafe { &*self.0.green_node }
    }

    fn green_node_children(&self) -> &Vec<GreenNodeOrToken> {
        self.green_node().children()
    }

    pub fn root<T>(green_node: GreenNode) -> Option<T>
    where
        T: AstNode,
    {
        let root = Box::new(green_node);

        T::cast(Self(Rc::new(SyntaxNodeInner {
            offset: 0,
            green_node: &*root as *const _,
            parent: Weak::new(),
            root_green_node: Some(root),
        })))
    }

    pub fn offset(&self) -> u32 {
        self.0.offset
    }

    pub fn range(&self) -> TextRange {
        let start = self.offset();
        let end = start
            + self
                .green_node_children()
                .iter()
                .map(|it| it.width())
                .sum::<u32>();
        TextRange::new(start.into(), end.into())
    }

    pub fn kind(&self) -> SyntaxKind {
        self.green_node().kind()
    }

    pub fn cast_token<T>(&self) -> Option<T>
    where
        T: AstToken,
    {
        let mut cur_offset = self.offset();

        for child in self.green_node_children().iter() {
            match child {
                GreenNodeOrToken::Token(green_node) if T::can_cast(green_node.kind()) => {
                    return T::cast(SyntaxToken {
                        offset: cur_offset,
                        kind: green_node.kind(),
                        value: green_node.value().to_string(),
                    });
                }
                _ => {
                    cur_offset += child.width();
                }
            }
        }

        None
    }

    pub fn cast_child<T>(&self) -> Option<T>
    where
        T: AstNode,
    {
        let mut cur_offset = self.offset();

        for child in self.green_node_children().iter() {
            match child {
                GreenNodeOrToken::Node(green_node) if T::can_cast(green_node.kind()) => {
                    let n = Self(Rc::new(SyntaxNodeInner {
                        offset: cur_offset,
                        green_node: green_node as *const _,
                        parent: Rc::<SyntaxNodeInner>::downgrade(&self.0),
                        root_green_node: None,
                    }));

                    return T::cast(n);
                }
                _ => {
                    cur_offset += child.width();
                }
            }
        }

        None
    }

    pub fn cast_children<T>(&self) -> Vec<T>
    where
        T: AstNode,
    {
        let mut cur_offset = self.offset();

        self.green_node_children()
            .iter()
            .filter_map(|child| {
                let offset = cur_offset;
                cur_offset += child.width();

                match child {
                    GreenNodeOrToken::Node(green_node) => {
                        let n = Self(Rc::new(SyntaxNodeInner {
                            offset,
                            green_node: green_node as *const _,
                            parent: Rc::<SyntaxNodeInner>::downgrade(&self.0),
                            root_green_node: None,
                        }));

                        T::cast(n)
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn id(&self) -> SyntaxNodeId {
        SyntaxNodeId {
            kind: self.kind(),
            offset: self.offset(),
        }
    }
}

#[derive(Debug)]
struct SyntaxNodeInner {
    offset: u32,
    green_node: *const GreenNode,
    parent: Weak<SyntaxNodeInner>,

    /// Owned by the root, making sure `green_node` is valid.
    #[allow(dead_code)]
    root_green_node: Option<Box<GreenNode>>,
}
