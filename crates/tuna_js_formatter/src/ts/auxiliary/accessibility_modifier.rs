use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::TsAccessibilityModifier;
use tuna_js_syntax::TsAccessibilityModifierFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsAccessibilityModifier;

impl FormatNodeRule<TsAccessibilityModifier> for FormatTsAccessibilityModifier {
    fn fmt_fields(&self, node: &TsAccessibilityModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAccessibilityModifierFields { modifier_token } = node.as_fields();

        write![f, [modifier_token.format()]]
    }
}
