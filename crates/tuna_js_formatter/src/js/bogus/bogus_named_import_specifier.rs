use crate::FormatBogusNodeRule;
use tuna_js_syntax::JsBogusNamedImportSpecifier;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusNamedImportSpecifier;

impl FormatBogusNodeRule<JsBogusNamedImportSpecifier> for FormatJsBogusNamedImportSpecifier {}
