use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::JsObjectBindingPatternRest;
use tuna_js_syntax::JsObjectBindingPatternRestFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectBindingPatternRest;

impl FormatNodeRule<JsObjectBindingPatternRest> for FormatJsObjectBindingPatternRest {
    fn fmt_fields(
        &self,
        node: &JsObjectBindingPatternRest,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectBindingPatternRestFields {
            dotdotdot_token,
            binding,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), binding.format(),]]
    }
}
