use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::JsObjectBindingPatternProperty;
use tuna_js_syntax::JsObjectBindingPatternPropertyFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectBindingPatternProperty;

impl FormatNodeRule<JsObjectBindingPatternProperty> for FormatJsObjectBindingPatternProperty {
    fn fmt_fields(
        &self,
        node: &JsObjectBindingPatternProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectBindingPatternPropertyFields {
            member,
            colon_token,
            pattern,
            init,
        } = node.as_fields();

        write![
            f,
            [
                member.format(),
                colon_token.format(),
                space(),
                pattern.format(),
            ]
        ]?;

        if let Some(init) = init {
            write!(f, [space(), init.format()])?;
        }

        Ok(())
    }
}
