use anyhow::Result;
use log::error;
use rustpython_parser::ast::Location;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ruff_python_ast::types::Range;

use crate::edit::Edit;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DiagnosticKind {
    /// The identifier of the diagnostic, used to align the diagnostic with a rule.
    pub name: String,
    /// The message body to display to the user, to explain the diagnostic.
    pub body: String,
    /// The message to display to the user, to explain the suggested fix.
    pub suggestion: Option<String>,
    /// Whether the diagnostic is automatically fixable.
    pub fixable: bool,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub location: Location,
    pub end_location: Location,
    pub fix: Option<Edit>,
    pub parent: Option<Location>,
}

impl Diagnostic {
    pub fn new<K: Into<DiagnosticKind>>(kind: K, range: Range) -> Self {
        Self {
            kind: kind.into(),
            location: range.location,
            end_location: range.end_location,
            fix: None,
            parent: None,
        }
    }

    pub fn amend(&mut self, fix: Edit) -> &mut Self {
        self.fix = Some(fix);
        self
    }

    pub fn try_amend(&mut self, func: impl FnOnce() -> Result<Edit>) -> &mut Self {
        match func() {
            Ok(fix) => self.fix = Some(fix),
            Err(err) => error!("Failed to create fix: {}", err),
        }
        self
    }

    pub fn parent(&mut self, parent: Location) -> &mut Self {
        self.parent = Some(parent);
        self
    }
}
