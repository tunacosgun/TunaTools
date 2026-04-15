use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::JsArrayBindingPatternRestElement;
use tuna_js_syntax::JsArrayBindingPatternRestElementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsArrayBindingPatternRestElement;

impl FormatNodeRule<JsArrayBindingPatternRestElement> for FormatJsArrayBindingPatternRestElement {
    fn fmt_fields(
        &self,
        node: &JsArrayBindingPatternRestElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsArrayBindingPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), pattern.format()]]
    }
}
