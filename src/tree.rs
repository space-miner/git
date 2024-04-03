use crate::blob::Kind;
use crate::entry::Entry;

#[derive(Debug)]
pub struct Tree {
    entries: Vec<Entry>,
    mode: String,
    kind: Kind,
}

impl Tree {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self {
            entries,
            mode: "100644".to_string(),
            kind: Kind::Tree,
        }
    }
}

use std::fmt;

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) {
        !todo()
    }
}
