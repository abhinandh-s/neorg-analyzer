use crate::error::MarkdownError;
use crate::node::CSTNode;

// Visitor trait for tree traversal
pub trait Visitor {
    fn visit_document(&mut self, node: &CSTNode) -> Result<(), MarkdownError>;
    fn visit_heading(&mut self, node: &CSTNode, level: u8) -> Result<(), MarkdownError>;
    fn visit_paragraph(&mut self, node: &CSTNode) -> Result<(), MarkdownError>;
    fn visit_text(&mut self, node: &CSTNode) -> Result<(), MarkdownError>;
    fn visit_bold(&mut self, node: &CSTNode) -> Result<(), MarkdownError>;
    fn visit_italic(&mut self, node: &CSTNode) -> Result<(), MarkdownError>;
    fn visit_line_break(&mut self, node: &CSTNode) -> Result<(), MarkdownError>;
}

// Formatting options
#[derive(Debug, Clone)]
pub struct FormattingOptions {
    pub indent_spaces: usize,
    pub line_width: usize,
    pub preserve_empty_lines: bool,
    pub heading_style: HeadingStyle,
    pub emphasis_style: EmphasisStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum HeadingStyle {
    Atx, // # Heading
    Setext, // Heading
         // =======
}

#[derive(Debug, Clone, Copy)]
pub enum EmphasisStyle {
    Asterisk,   // *italic* **bold**
    Underscore, // _italic_ __bold__
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            indent_spaces: 4,
            line_width: 80,
            preserve_empty_lines: true,
            heading_style: HeadingStyle::Atx,
            emphasis_style: EmphasisStyle::Asterisk,
        }
    }
}

// Formatter visitor
pub struct MarkdownFormatter {
    pub options: FormattingOptions,
    pub output: String,
    pub indent_level: usize,
}

impl MarkdownFormatter {
    pub fn new(options: FormattingOptions) -> Self {
        Self {
            options,
            output: String::new(),
            indent_level: 0,
        }
    }

    pub fn format(&mut self, node: &CSTNode) -> Result<String, MarkdownError> {
        self.visit_node(node)?;
        Ok(self.output.clone())
    }

    fn visit_node(&mut self, node: &CSTNode) -> Result<(), MarkdownError> {
        match node {
            CSTNode::Document { .. } => self.visit_document(node),
            CSTNode::Heading { level, .. } => self.visit_heading(node, *level),
            CSTNode::Paragraph { .. } => self.visit_paragraph(node),
            CSTNode::Text { .. } => self.visit_text(node),
            CSTNode::Bold { .. } => self.visit_bold(node),
            CSTNode::Italic { .. } => self.visit_italic(node),
            CSTNode::LineBreak { .. } => self.visit_line_break(node),
            _ => Ok(()),
        }
    }
}

impl Visitor for MarkdownFormatter {
    fn visit_document(&mut self, node: &CSTNode) -> Result<(), MarkdownError> {
        if let CSTNode::Document { children, .. } = node {
            for (i, child) in children.iter().enumerate() {
                if i > 0 {
                    self.output.push('\n');
                    if self.options.preserve_empty_lines {
                        self.output.push('\n');
                    }
                }
                self.visit_node(child)?;
            }
        }
        Ok(())
    }

    fn visit_heading(&mut self, node: &CSTNode, level: u8) -> Result<(), MarkdownError> {
        if let CSTNode::Heading { content, .. } = node {
            match self.options.heading_style {
                HeadingStyle::Atx => {
                    self.output.push_str(&"#".repeat(level as usize));
                    self.output.push(' ');
                    for (i, child) in content.iter().enumerate() {
                        if i > 0 {
                            self.output.push(' ');
                        }
                        self.visit_node(child)?;
                    }
                }
                HeadingStyle::Setext => {
                    for (i, child) in content.iter().enumerate() {
                        if i > 0 {
                            self.output.push(' ');
                        }
                        self.visit_node(child)?;
                    }
                    self.output.push('\n');
                    self.output
                        .push_str(&if level == 1 { "=" } else { "-" }.repeat(content.len()));
                }
            }
        }
        Ok(())
    }

    fn visit_paragraph(&mut self, node: &CSTNode) -> Result<(), MarkdownError> {
        if let CSTNode::Paragraph { content, .. } = node {
            for child in content {
                self.visit_node(child)?;
            }
        }
        Ok(())
    }

    fn visit_text(&mut self, node: &CSTNode) -> Result<(), MarkdownError> {
        if let CSTNode::Text { content, .. } = node {
            self.output.push_str(content);
        }
        Ok(())
    }

    fn visit_bold(&mut self, node: &CSTNode) -> Result<(), MarkdownError> {
        if let CSTNode::Bold { content, .. } = node {
            self.output.push_str("**");
            for child in content {
                self.visit_node(child)?;
            }
            self.output.push_str("**");
        }
        Ok(())
    }

    fn visit_italic(&mut self, node: &CSTNode) -> Result<(), MarkdownError> {
        if let CSTNode::Italic { content, .. } = node {
            self.output.push('*');
            for child in content {
                self.visit_node(child)?;
            }
            self.output.push('*');
        }
        Ok(())
    }

    fn visit_line_break(&mut self, _node: &CSTNode) -> Result<(), MarkdownError> {
        self.output.push('\n');
        Ok(())
    }
}
