use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::TsQualifiedModuleName;
use tuna_js_syntax::TsQualifiedModuleNameFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsQualifiedModuleName;

impl FormatNodeRule<TsQualifiedModuleName> for FormatTsQualifiedModuleName {
    fn fmt_fields(&self, node: &TsQualifiedModuleName, f: &mut JsFormatter) -> FormatResult<()> {
        let TsQualifiedModuleNameFields {
            left,
            dot_token,
            right,
        } = node.as_fields();

        write![f, [left.format(), dot_token.format(), right.format(),]]
    }
}
