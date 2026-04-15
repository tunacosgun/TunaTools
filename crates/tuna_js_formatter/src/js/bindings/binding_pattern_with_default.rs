use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::JsBindingPatternWithDefault;
use tuna_js_syntax::JsBindingPatternWithDefaultFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBindingPatternWithDefault;

impl FormatNodeRule<JsBindingPatternWithDefault> for FormatJsBindingPatternWithDefault {
    fn fmt_fields(
        &self,
        node: &JsBindingPatternWithDefault,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsBindingPatternWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = node.as_fields();

        write![
            f,
            [
                pattern.format(),
                space(),
                eq_token.format(),
                space(),
                default.format()
            ]
        ]
    }
}
