use crate::node::{ASTNode, CSTNode};

// Implement Display and detailed pretty printing for CST
impl CSTNode {
    pub fn pretty_print(&self) -> String {
        let mut output = String::new();
        self.pretty_print_internal(&mut output, 0);
        output
    }

    fn pretty_print_internal(&self, output: &mut String, depth: usize) {
        let indent = "  ".repeat(depth);
        match self {
            CSTNode::Document { children, span } => {
                output.push_str(&format!("{}Document (span: {:?}) {{\n", indent, span));
                for child in children {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            CSTNode::Heading {
                level,
                content,
                markers,
                span,
            } => {
                output.push_str(&format!(
                    "{}Heading Level {} (span: {:?}, markers: {:?}) {{\n",
                    indent, level, span, markers
                ));
                for child in content {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            CSTNode::Paragraph { content, span } => {
                output.push_str(&format!("{}Paragraph (span: {:?}) {{\n", indent, span));
                for child in content {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            CSTNode::Text { content, span } => {
                output.push_str(&format!(
                    "{}Text (span: {:?}): \"{}\"\n",
                    indent, span, content
                ));
            }
            CSTNode::Bold {
                content,
                markers,
                span,
            } => {
                output.push_str(&format!(
                    "{}Bold (span: {:?}, markers: {:?}) {{\n",
                    indent, span, markers
                ));
                for child in content {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            CSTNode::Italic {
                content,
                markers,
                span,
            } => {
                output.push_str(&format!(
                    "{}Italic (span: {:?}, markers: {:?}) {{\n",
                    indent, span, markers
                ));
                for child in content {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            CSTNode::LineBreak { span } => {
                output.push_str(&format!("{}LineBreak (span: {:?})\n", indent, span));
            }
            _ => {}
        }
    }
}

// Implement Display and detailed pretty printing for AST
impl ASTNode {
    pub fn pretty_print(&self) -> String {
        let mut output = String::new();
        self.pretty_print_internal(&mut output, 0);
        output
    }

    fn pretty_print_internal(&self, output: &mut String, depth: usize) {
        let indent = "  ".repeat(depth);
        match self {
            ASTNode::Document(children) => {
                output.push_str(&format!("{}Document {{\n", indent));
                for child in children {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            ASTNode::Heading { level, content } => {
                output.push_str(&format!("{}Heading Level {} {{\n", indent, level));
                for child in content {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            ASTNode::Paragraph(content) => {
                output.push_str(&format!("{}Paragraph {{\n", indent));
                for child in content {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            ASTNode::Text(content) => {
                output.push_str(&format!("{}Text: \"{}\"\n", indent, content));
            }
            ASTNode::Bold(content) => {
                output.push_str(&format!("{}Bold {{\n", indent));
                for child in content {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            ASTNode::Italic(content) => {
                output.push_str(&format!("{}Italic {{\n", indent));
                for child in content {
                    child.pretty_print_internal(output, depth + 1);
                }
                output.push_str(&format!("{}}}\n", indent));
            }
            ASTNode::LineBreak => {
                output.push_str(&format!("{}LineBreak\n", indent));
            }
            ASTNode::Error(_) => {},
        }
    }
}
