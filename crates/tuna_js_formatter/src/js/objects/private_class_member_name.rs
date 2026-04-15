use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::JsPrivateClassMemberName;
use tuna_js_syntax::JsPrivateClassMemberNameFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPrivateClassMemberName;

impl FormatNodeRule<JsPrivateClassMemberName> for FormatJsPrivateClassMemberName {
    fn fmt_fields(&self, node: &JsPrivateClassMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPrivateClassMemberNameFields {
            hash_token,
            id_token,
        } = node.as_fields();

        write![f, [hash_token.format(), id_token.format()]]
    }
}
