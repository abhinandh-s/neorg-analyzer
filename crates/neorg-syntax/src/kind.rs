use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // Structural tokens
    Heading { level: usize },         // *
    ReverseHeading { levels: usize }, // --- or ===

    // List tokens
    UnorderedList { level: usize }, // -
    OrderedList { level: usize },   // ~

    // Task status
    TaskMarkerStart,        // |(
    TaskMarkerEnd,          // )|
    TaskStatus(TaskStatus), // Various status symbols

    // Code blocks
    CodeBlockStart { language: Option<String> }, // @code [lang]
    CodeBlockEnd,                                // @end

    // Markup tokens
    Bold,          // *text*
    Italic,        // /text/
    Underline,     // _text_
    Strikethrough, // -text-
    Spoiler,       // !text!
    Verbatim,      // text
    Comment,       // %text%

    // Link tokens
    LinkStart,          // {
    LinkEnd,            // }
    LinkDescStart,      // [
    LinkDescEnd,        // ]
    LinkModifier(char), // #, *, /, etc.

    // Text content
    Text(String),
    Whitespace(String),
    Newline,

    // Special characters
    Pipe, // |

    // End of file
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    Undone,     // ( )
    Done,       // (x)
    NeedsInput, // (?)
    Urgent,     // (!)
    Recurring,  // (+)
    Pending,    // (-)
    OnHold,     // (=)
    Cancelled,  // (_)
}

impl FromStr for TaskStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            " " => Ok(TaskStatus::Undone),
            "x" => Ok(TaskStatus::Done),
            "?" => Ok(TaskStatus::NeedsInput),
            "!" => Ok(TaskStatus::Urgent),
            "+" => Ok(TaskStatus::Recurring),
            "-" => Ok(TaskStatus::Pending),
            "=" => Ok(TaskStatus::OnHold),
            "_" => Ok(TaskStatus::Cancelled),
            _ => Err(()),
        }
    }
}
