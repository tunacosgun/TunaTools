use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::JsReferenceIdentifier;
use tuna_js_syntax::JsReferenceIdentifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsReferenceIdentifier;

impl FormatNodeRule<JsReferenceIdentifier> for FormatJsReferenceIdentifier {
    fn fmt_fields(&self, node: &JsReferenceIdentifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsReferenceIdentifierFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
