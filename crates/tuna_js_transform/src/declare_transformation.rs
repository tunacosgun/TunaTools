#[macro_export]
macro_rules! declare_transformation {
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident {
        version: $version:literal,
        name: $name:tt,
        $( $key:ident: $value:expr, )*
    } ) => {
        $( #[doc = $doc] )*
        $vis enum $id {}

        impl ::tuna_analyze::RuleMeta for $id {
            type Group = $crate::registry::TransformationGroup;
            const METADATA: ::tuna_analyze::RuleMetadata =
                ::tuna_analyze::RuleMetadata::new($version, $name, concat!( $( $doc, "\n", )* ));
        }
    };
}
