use crate::FormatBogusNodeRule;
use tuna_js_syntax::JsBogus;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogus;

impl FormatBogusNodeRule<JsBogus> for FormatJsBogus {}
