use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::JsShorthandPropertyObjectMember;
use tuna_js_syntax::JsShorthandPropertyObjectMemberFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsShorthandPropertyObjectMember;

impl FormatNodeRule<JsShorthandPropertyObjectMember> for FormatJsShorthandPropertyObjectMember {
    fn fmt_fields(
        &self,
        node: &JsShorthandPropertyObjectMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsShorthandPropertyObjectMemberFields { name } = node.as_fields();

        write![f, [name.format()]]
    }
}
