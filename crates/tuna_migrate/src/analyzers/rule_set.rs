use crate::declare_migration;
use tuna_analyze::context::RuleContext;
use tuna_analyze::{Ast, Rule};
use tuna_json_syntax::JsonObjectValue;

declare_migration! {
    pub(crate) RuleSet {
        version: "11.0.0",
        name: "ruleSet",
    }
}

impl Rule for RuleSet {
    type Query = Ast<JsonObjectValue>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        // TODO: write rule to create a "ruleSet" config
        // ruleSet -> "recommended", "all", "none" as a starter
        // It should merge "recommended" and "all"
        None
    }
}
