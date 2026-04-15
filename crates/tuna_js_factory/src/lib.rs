use tuna_js_syntax::JsLanguage;
use tuna_rowan::TreeBuilder;

mod generated;
pub use crate::generated::JsSyntaxFactory;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use tuna_js_syntax as syntax;

pub type JsSyntaxTreeBuilder = TreeBuilder<'static, JsLanguage, JsSyntaxFactory>;
