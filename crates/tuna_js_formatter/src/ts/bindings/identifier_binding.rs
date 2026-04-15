use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::{TsIdentifierBinding, TsIdentifierBindingFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsIdentifierBinding;

impl FormatNodeRule<TsIdentifierBinding> for FormatTsIdentifierBinding {
    fn fmt_fields(&self, node: &TsIdentifierBinding, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIdentifierBindingFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
