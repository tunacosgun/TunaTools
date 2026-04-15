//! This module defines the Concrete Syntax Tree used by Rome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{AnyJsRoot, JsSyntaxKind};
use tuna_rowan::Language;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, schemars::JsonSchema))]
pub struct JsLanguage;

impl Language for JsLanguage {
    type Kind = JsSyntaxKind;
    type Root = AnyJsRoot;
}

pub type JsSyntaxNode = tuna_rowan::SyntaxNode<JsLanguage>;
pub type JsSyntaxToken = tuna_rowan::SyntaxToken<JsLanguage>;
pub type JsSyntaxElement = tuna_rowan::SyntaxElement<JsLanguage>;
pub type JsSyntaxNodeChildren = tuna_rowan::SyntaxNodeChildren<JsLanguage>;
pub type JsSyntaxElementChildren = tuna_rowan::SyntaxElementChildren<JsLanguage>;
pub type JsSyntaxList = tuna_rowan::SyntaxList<JsLanguage>;
pub type JsSyntaxTrivia = tuna_rowan::syntax::SyntaxTrivia<JsLanguage>;
