use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::JsPrivateName;
use tuna_js_syntax::JsPrivateNameFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPrivateName;

impl FormatNodeRule<JsPrivateName> for FormatJsPrivateName {
    fn fmt_fields(&self, node: &JsPrivateName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPrivateNameFields {
            hash_token,
            value_token,
        } = node.as_fields();

        write![f, [hash_token.format(), value_token.format()]]
    }
}
