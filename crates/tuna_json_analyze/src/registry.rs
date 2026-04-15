//! Generated file, do not edit by hand, see `xtask/codegen`

use tuna_analyze::RegistryVisitor;
use tuna_json_syntax::JsonLanguage;
pub fn visit_registry<V: RegistryVisitor<JsonLanguage>>(registry: &mut V) {
    registry.record_category::<crate::analyzers::Analyzers>();
}
