use crate::prelude::*;
use crate::utils::AnyJsAssignmentLike;

use tuna_formatter::write;
use tuna_js_syntax::JsPropertyObjectMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPropertyObjectMember;

impl FormatNodeRule<JsPropertyObjectMember> for FormatJsPropertyObjectMember {
    fn fmt_fields(&self, node: &JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [AnyJsAssignmentLike::from(node.clone())]]
    }
}
