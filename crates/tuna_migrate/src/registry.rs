use crate::analyzers::MigrationCategory;
use tuna_analyze::RegistryVisitor;
use tuna_json_syntax::JsonLanguage;

pub fn visit_migration_registry<V: RegistryVisitor<JsonLanguage>>(registry: &mut V) {
    registry.record_category::<MigrationCategory>();
}
