use crate::error::MarkdownError;
use crate::node::CSTNode;

// Validation rules
pub struct ValidationRule {
    name: String,
    check: Box<dyn Fn(&CSTNode) -> bool>,
    message: String,
}

impl ValidationRule {
    pub fn new(
        name: impl Into<String>,
        check: impl Fn(&CSTNode) -> bool + 'static,
        message: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            check: Box::new(check),
            message: message.into(),
        }
    }
}

// Validator visitor
pub struct MarkdownValidator {
    rules: Vec<ValidationRule>,
    errors: Vec<String>,
}

impl MarkdownValidator {
    pub fn new() -> Self {
        let default_rules = vec![
            ValidationRule::new(
                "heading_level",
                |node| {
                    if let CSTNode::Heading { level, .. } = node {
                        *level <= 6
                    } else {
                        true
                    }
                },
                "Heading level must be between 1 and 6",
            ),
            ValidationRule::new(
                "empty_heading",
                |node| {
                    if let CSTNode::Heading { content, .. } = node {
                        !content.is_empty()
                    } else {
                        true
                    }
                },
                "Headings must not be empty",
            ),
            // Add more validation rules
        ];

        Self {
            rules: default_rules,
            errors: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.push(rule);
    }

    pub fn validate(&mut self, node: &CSTNode) -> Result<(), MarkdownError> {
        self.visit_node(node)?;
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(MarkdownError::ValidationError(self.errors.join("\n")))
        }
    }

    fn visit_node(&mut self, node: &CSTNode) -> Result<(), MarkdownError> {
        // Apply validation rules
        for rule in &self.rules {
            if !(rule.check)(node) {
                self.errors.push(format!("{}: {}", rule.name, rule.message));
            }
        }

        // Recursively validate children
        match node {
            CSTNode::Document { children, .. } => {
                for child in children {
                    self.visit_node(child)?;
                }
            }
            CSTNode::Heading { content, .. } => {
                for child in content {
                    self.visit_node(child)?;
                }
            }
            CSTNode::Paragraph {
                content: _,
                span: _,
            } => {}
            CSTNode::Text {
                content: _,
                span: _,
            } => {}
            CSTNode::Bold {
                content,
                markers: _,
                span: _,
            } => {
                for child in content {
                    self.visit_node(child)?;
                }
            }
            CSTNode::Italic {
                content,
                markers: _,
                span: _,
            } => {
                for child in content {
                    self.visit_node(child)?;
                }
            }
            CSTNode::LineBreak { span: _ } => {}
            CSTNode::Error { text: _, error: _ } => {},
        }

        Ok(())
    }
}

impl Default for MarkdownValidator {
    fn default() -> Self {
        Self::new()
    }
}
