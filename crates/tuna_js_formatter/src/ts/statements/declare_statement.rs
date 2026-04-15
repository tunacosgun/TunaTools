use crate::prelude::*;

use tuna_formatter::write;
use tuna_js_syntax::TsDeclareStatement;
use tuna_js_syntax::TsDeclareStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsDeclareStatement;

impl FormatNodeRule<TsDeclareStatement> for FormatTsDeclareStatement {
    fn fmt_fields(&self, node: &TsDeclareStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let TsDeclareStatementFields {
            declaration,
            declare_token,
        } = node.as_fields();
        write![f, [declare_token.format(), space(), declaration.format()]]
    }
}
