use crate::FormatBogusNodeRule;
use tuna_json_syntax::JsonBogus;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonBogus;

impl FormatBogusNodeRule<JsonBogus> for FormatJsonBogus {}
