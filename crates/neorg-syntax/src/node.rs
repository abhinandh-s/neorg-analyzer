use ecow::{EcoString, EcoVec};

use crate::kind::SyntaxKind;
use crate::span::Span;

/// A syntactical error.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxError {
    /// The node's span.
    pub span: Span,
    /// The error message.
    pub message: EcoString,
    /// Additional hints to the user, indicating how this error could be avoided
    /// or worked around.
    pub hints: EcoVec<EcoString>,
}

#[derive(Debug)]
pub enum CSTNode {
    Document {
        children: Vec<CSTNode>,
        span: Span,
    },
    Heading {
        level: u8,
        content: Vec<CSTNode>,
        markers: Span, // The '#' characters
        span: Span,
    },
    Paragraph {
        content: Vec<CSTNode>,
        span: Span,
    },
    Text {
        content: String,
        span: Span,
    },
    Bold {
        content: Vec<CSTNode>,
        markers: Span, // The '**' or '__' markers
        span: Span,
    },
    Italic {
        content: Vec<CSTNode>,
        markers: Span, // The '*' or '_' markers
        span: Span,
    },
    LineBreak {
        span: Span,
    },
    Error {
        text: EcoString,
        error: SyntaxError,
    },
}

impl CSTNode {
    /// Get the span of a nodes text excluding the markers
    pub fn text_span(&self) -> Span {
        match self {
            CSTNode::Document { .. } => Span::new(0, 0, self.span().line, self.span().column),
            CSTNode::Heading { markers, .. } => {
                let start = markers.start;
                let end = markers.end + 1;
                let r = end - start;
                Span::new(start + r, end, self.span().line, self.span().column)
            }
            CSTNode::Paragraph { content, .. } => {
                let start = content
                    .iter()
                    .map(|n| n.text_span().start)
                    .min()
                    .unwrap_or(0);
                let end = content
                    .iter()
                    .map(|n| n.text_span().end)
                    .max()
                    .unwrap_or(start);
                Span::new(start, end, self.span().line, self.span().column)
            }
            CSTNode::Text { span, .. } => span.clone(),
            CSTNode::Bold {
                content, markers, ..
            } => {
                let start = markers.end;
                let end = content
                    .iter()
                    .map(|n| n.text_span().end)
                    .max()
                    .unwrap_or(start);
                Span::new(start, end, self.span().line, self.span().column)
            }
            CSTNode::Italic {
                content, markers, ..
            } => {
                let start = markers.end;
                let end = content
                    .iter()
                    .map(|n| n.text_span().end)
                    .max()
                    .unwrap_or(start);
                Span::new(start, end, self.span().line, self.span().column)
            }
            CSTNode::LineBreak { span, .. } => span.clone(),
            CSTNode::Error { error, .. } => error.span.clone(),
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        match self {
            CSTNode::Document { .. } => SyntaxKind::Document,
            CSTNode::Heading { .. } => SyntaxKind::Heading,
            CSTNode::Paragraph { .. } => SyntaxKind::Paragraph,
            CSTNode::Text { .. } => SyntaxKind::Text,
            CSTNode::Bold { .. } => SyntaxKind::Bold,
            CSTNode::Italic { .. } => SyntaxKind::Italic,
            CSTNode::LineBreak { .. } => SyntaxKind::LineBreak,
            CSTNode::Error { .. } => SyntaxKind::Error,
        }
    }

    pub fn traverse_for<'a>(&'a self, nodes: &mut Vec<&'a CSTNode>) {
        nodes.push(self);
        if let Some(children) = self.children() {
            for child in children {
                child.traverse_for(nodes);
            }
        }
    }

    pub fn find_by_kind(&self, kind: SyntaxKind) -> Vec<&CSTNode> {
        let mut all_nodes = Vec::new();
        self.traverse_for(&mut all_nodes);
        all_nodes
            .into_iter()
            .filter(|node| node.kind() == kind)
            .collect()
    }

    pub fn find_by_span(&self, span: &Span) -> Vec<&CSTNode> {
        let mut all_nodes = Vec::new();
        self.traverse_for(&mut all_nodes);
        all_nodes
            .into_iter()
            .filter(|node| node.span() == span)
            .collect()
    }

    pub fn traverse<F>(&self, f: &mut F)
    where
        F: FnMut(&CSTNode),
    {
        f(self);
        if let Some(children) = self.children() {
            for child in children {
                child.traverse(f);
            }
        }
    }

    /// Get the children of a node
    pub fn children(&self) -> Option<&Vec<CSTNode>> {
        match self {
            CSTNode::Heading { content, .. } => Some(content),
            CSTNode::Paragraph { content, .. } => Some(content),
            CSTNode::Bold { content, .. } => Some(content),
            CSTNode::Italic { content, .. } => Some(content),
            CSTNode::Document { children, span: _ } => Some(children),
            _ => None,
        }
    }

    /// Get the span of a node
    pub fn span(&self) -> &Span {
        match self {
            CSTNode::Document { span, .. } => span,
            CSTNode::Heading { span, .. } => span,
            CSTNode::Paragraph { span, .. } => span,
            CSTNode::Text { span, .. } => span,
            CSTNode::Bold { span, .. } => span,
            CSTNode::Italic { span, .. } => span,
            CSTNode::LineBreak { span, .. } => span,
            CSTNode::Error { text: _, error } => &error.span,
        }
    }

    /// Get the text content of a Text node
    pub fn text(&self) -> Option<&str> {
        match self {
            CSTNode::Text { content, .. } => Some(content),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ASTNode {
    Document(Vec<ASTNode>),
    Heading { level: u8, content: Vec<ASTNode> },
    Paragraph(Vec<ASTNode>),
    Text(String),
    Bold(Vec<ASTNode>),
    Italic(Vec<ASTNode>),
    LineBreak,
    Error(EcoString),
}

impl ASTNode {
    pub fn traverse<F>(&self, f: &mut F)
    where
        F: FnMut(&ASTNode),
    {
        f(self);
        if let Some(children) = self.children() {
            for child in children {
                child.traverse(f);
            }
        }
    }
    /// Get the children of a node
    pub fn children(&self) -> Option<&Vec<ASTNode>> {
        match self {
            ASTNode::Document(children) => Some(children),
            ASTNode::Heading { content, .. } => Some(content),
            ASTNode::Paragraph(content) => Some(content),
            ASTNode::Bold(content) => Some(content),
            ASTNode::Italic(content) => Some(content),
            _ => None,
        }
    }

    /// Get the text content of a Text node
    pub fn text(&self) -> Option<&str> {
        match self {
            ASTNode::Text(content) => Some(content),
            _ => None,
        }
    }
}
