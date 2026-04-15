use crate::prelude::*;
use tuna_formatter::write;

use tuna_js_syntax::JsExportAsClause;
use tuna_js_syntax::JsExportAsClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportAsClause;

impl FormatNodeRule<JsExportAsClause> for FormatJsExportAsClause {
    fn fmt_fields(&self, node: &JsExportAsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportAsClauseFields {
            as_token,
            exported_name,
        } = node.as_fields();

        write![f, [as_token.format(), space(), exported_name.format()]]
    }
}
