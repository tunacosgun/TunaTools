use crate::FormatBogusNodeRule;
use tuna_json_syntax::JsonBogusValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonBogusValue;

impl FormatBogusNodeRule<JsonBogusValue> for FormatJsonBogusValue {}
