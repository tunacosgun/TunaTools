use crate::prelude::*;
use tuna_formatter::write;

use crate::js::declarations::function_declaration::FormatFunction;
use tuna_js_syntax::JsFunctionExportDefaultDeclaration;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsFunctionExportDefaultDeclaration;

impl FormatNodeRule<JsFunctionExportDefaultDeclaration>
    for FormatJsFunctionExportDefaultDeclaration
{
    fn fmt_fields(
        &self,
        node: &JsFunctionExportDefaultDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write![f, [FormatFunction::from(node.clone())]]
    }
}
