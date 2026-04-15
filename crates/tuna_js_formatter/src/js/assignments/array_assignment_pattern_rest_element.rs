use crate::prelude::*;
use tuna_formatter::write;

use tuna_js_syntax::JsArrayAssignmentPatternRestElement;
use tuna_js_syntax::JsArrayAssignmentPatternRestElementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsArrayAssignmentPatternRestElement;

impl FormatNodeRule<JsArrayAssignmentPatternRestElement>
    for FormatJsArrayAssignmentPatternRestElement
{
    fn fmt_fields(
        &self,
        node: &JsArrayAssignmentPatternRestElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsArrayAssignmentPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = node.as_fields();

        write!(f, [dotdotdot_token.format(), pattern.format()])
    }
}
