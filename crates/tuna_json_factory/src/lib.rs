pub use crate::generated::JsonSyntaxFactory;
use tuna_json_syntax::JsonLanguage;
use tuna_rowan::TreeBuilder;

mod generated;

// Re-exported for tests
#[doc(hidden)]
pub use tuna_json_syntax as syntax;

pub type JsonSyntaxTreeBuilder = TreeBuilder<'static, JsonLanguage, JsonSyntaxFactory>;

pub use generated::node_factory as make;
