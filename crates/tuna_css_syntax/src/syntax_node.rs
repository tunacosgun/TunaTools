//! This module defines the Concrete Syntax Tree used by Rome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{CssRoot, CssSyntaxKind};
use tuna_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CssLanguage;

impl Language for CssLanguage {
    type Kind = CssSyntaxKind;
    type Root = CssRoot;
}

pub type CssSyntaxNode = tuna_rowan::SyntaxNode<CssLanguage>;
pub type CssSyntaxToken = tuna_rowan::SyntaxToken<CssLanguage>;
pub type CssSyntaxElement = tuna_rowan::SyntaxElement<CssLanguage>;
pub type CssSyntaxNodeChildren = tuna_rowan::SyntaxNodeChildren<CssLanguage>;
pub type CssSyntaxElementChildren = tuna_rowan::SyntaxElementChildren<CssLanguage>;
pub type CssSyntaxList = tuna_rowan::SyntaxList<CssLanguage>;
