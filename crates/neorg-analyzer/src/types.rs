#![allow(dead_code)]

use std::ops::Deref;

use serde::Deserialize;

pub struct PlainText {
    ctx: ecow::EcoString,
}

impl PlainText {
    pub fn new(ctx: ecow::EcoString) -> Self {
        Self { ctx }
    }

    pub fn ctx(&self) -> &str {
        &self.ctx
    }

    pub fn set_ctx(&mut self, ctx: ecow::EcoString) {
        self.ctx = ctx;
    }

    pub fn push(&mut self, c: char) {
        self.ctx.push(c)
    }

    pub fn push_str(&mut self, string: &str) {
        self.ctx.push_str(string)
    }
}

pub type Synonym<'a> = &'a str;

#[derive(Debug, Deserialize)]
pub(crate) struct DictionaryEntry {
    pub(crate) word: String,
    pub(crate) phonetic: Option<String>,
    pub(crate) phonetics: Vec<Phonetic>,
    pub(crate) origin: Option<String>,
    pub(crate) meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Phonetic {
    pub(crate) text: Option<String>,
    pub(crate) audio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub(crate) part_of_speech: String,
    pub(crate) definitions: Vec<Definition>,
    pub(crate) synonyms: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct Definition {
    pub(crate) definition: String,
    pub(crate) example: Option<String>,
    pub(crate) synonyms: Vec<String>,
    pub(crate) antonyms: Vec<String>,
}

pub(crate) struct MarkDown(pub(crate) String);

impl Deref for MarkDown {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct Neorg(pub(crate) String);

impl Deref for Neorg {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Meaning> for MarkDown {
    fn from(value: &Meaning) -> Self {
        let mut md = String::new();
        let heading = format!("## {}\n\n", value.part_of_speech);
        md.push_str(&heading);
        let mut i = 1;
        for definition in &value.definitions {
            let MarkDown(content) = definition.into();
            md.push_str(&format!("{i}. {content}"));
            if !md.ends_with('\n') {
                md.push('\n');
            }
            i += 1;
        }
        Self(md)
    }
}

impl From<&Definition> for MarkDown {
    fn from(value: &Definition) -> Self {
        let mut md = String::new();
        md.push_str(value.definition.as_ref());
        md.push_str("\n \n");
        if let Some(example) = &value.example {
            md.push_str("> ### Example: \n");
            md.push_str(">     - ");
            md.push_str(example.as_ref());
            md.push('\n');
            md.push('\n');
        }
        Self(md)
    }
}

impl From<&DictionaryEntry> for MarkDown {
    fn from(value: &DictionaryEntry) -> Self {
        let mut md = String::new();
        md.push_str("# ");
        md.push_str(value.word.as_str());
        if let Some(phonetic) = &value.phonetic {
            md.push('\t');
            md.push_str(phonetic);
        }
        md.push('\n');
        md.push('\n');
        if let Some(origin) = &value.origin {
            md.push_str("## Origin\n");
            md.push('\t');
            md.push_str(origin);
            md.push('\n');
        }
        for i in &value.meanings {
            let MarkDown(content) = i.into();
            md.push_str(&content);
        }
        Self(md)
    }
}

impl From<&Meaning> for Neorg {
    fn from(value: &Meaning) -> Self {
        let mut md = String::new();
        let heading = format!("** {}\n\n", value.part_of_speech);
        md.push_str(&heading);
        let mut i = 1;
        for definition in &value.definitions {
            let MarkDown(content) = definition.into();
            md.push_str(&format!("{i}. {content}"));
            if !md.ends_with('\n') {
                md.push('\n');
            }
            i += 1;
        }
        Self(md)
    }
}

impl From<&Definition> for Neorg {
    fn from(value: &Definition) -> Self {
        let mut md = String::new();
        md.push_str(value.definition.as_ref());
        md.push_str("\n \n");
        if let Some(example) = &value.example {
            md.push_str("> *** Example: \n");
            md.push_str(">     - ");
            md.push_str(example.as_ref());
            md.push('\n');
            md.push('\n');
        }
        Self(md)
    }
}

impl From<&DictionaryEntry> for Neorg {
    fn from(value: &DictionaryEntry) -> Self {
        let mut md = String::new();
        md.push_str("* ");
        md.push_str(value.word.as_str());
        if let Some(phonetic) = &value.phonetic {
            md.push('\t');
            md.push_str(phonetic);
        }
        md.push('\n');
        md.push('\n');
        if let Some(origin) = &value.origin {
            md.push_str("** Origin\n");
            md.push('\t');
            md.push_str(origin);
            md.push('\n');
        }
        for i in &value.meanings {
            let MarkDown(content) = i.into();
            md.push_str(&content);
        }
        Self(md)
    }
}

#[macro_export]
macro_rules! position {
    () => {
        tower_lsp::lsp_types::Position {
            line: 0,
            character: 0,
        }
    };
}

#[macro_export]
macro_rules! range {
    () => {
        tower_lsp::lsp_types::Range {
            start: tower_lsp::lsp_types::Position {
                line: 0,
                character: 0,
            },
            end: tower_lsp::lsp_types::Position {
                line: 0,
                character: 0,
            },
        }
    };
}
