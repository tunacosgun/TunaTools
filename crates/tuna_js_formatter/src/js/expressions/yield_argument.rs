use crate::prelude::*;
use tuna_formatter::write;

use tuna_js_syntax::JsYieldArgument;
use tuna_js_syntax::JsYieldArgumentFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsYieldArgument;

impl FormatNodeRule<JsYieldArgument> for FormatJsYieldArgument {
    fn fmt_fields(&self, node: &JsYieldArgument, f: &mut JsFormatter) -> FormatResult<()> {
        let JsYieldArgumentFields {
            star_token,
            expression,
        } = node.as_fields();

        write![f, [star_token.format(), space(), expression.format()]]
    }
}
