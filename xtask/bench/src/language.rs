use crate::test_case::TestCase;
use criterion::black_box;
use tuna_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleCategories};
use tuna_formatter::{FormatResult, Formatted, PrintResult, Printed};
use tuna_js_analyze::analyze;
use tuna_js_formatter::context::{JsFormatContext, JsFormatOptions};
use tuna_js_parser::JsParserOptions;
use tuna_js_syntax::{AnyJsRoot, JsFileSource, JsSyntaxNode};
use tuna_json_formatter::context::{JsonFormatContext, JsonFormatOptions};
use tuna_json_parser::JsonParserOptions;
use tuna_json_syntax::JsonSyntaxNode;
use tuna_parser::prelude::ParseDiagnostic;
use tuna_rowan::NodeCache;

pub enum Parse<'a> {
    JavaScript(JsFileSource, &'a str),
    Json(&'a str),
}

impl<'a> Parse<'a> {
    pub fn try_from_case(case: &TestCase) -> Option<Parse> {
        match JsFileSource::try_from(case.path()) {
            Ok(source_type) => Some(Parse::JavaScript(source_type, case.code())),
            Err(_) => match case.extension() {
                "json" => Some(Parse::Json(case.code())),
                _ => None,
            },
        }
    }

    pub fn parse(&self) -> Parsed {
        match self {
            Parse::JavaScript(source_type, code) => Parsed::JavaScript(
                tuna_js_parser::parse(code, *source_type, JsParserOptions::default()),
                *source_type,
            ),
            Parse::Json(code) => Parsed::Json(tuna_json_parser::parse_json(
                code,
                JsonParserOptions::default(),
            )),
        }
    }

    pub fn parse_with_cache(&self, cache: &mut NodeCache) -> Parsed {
        match self {
            Parse::JavaScript(source_type, code) => Parsed::JavaScript(
                tuna_js_parser::parse_js_with_cache(
                    code,
                    *source_type,
                    JsParserOptions::default(),
                    cache,
                ),
                *source_type,
            ),
            Parse::Json(code) => Parsed::Json(tuna_json_parser::parse_json_with_cache(
                code,
                cache,
                JsonParserOptions::default(),
            )),
        }
    }
}

pub enum Parsed {
    JavaScript(tuna_js_parser::Parse<AnyJsRoot>, JsFileSource),
    Json(tuna_json_parser::JsonParse),
}

impl Parsed {
    pub fn format_node(&self) -> Option<FormatNode> {
        match self {
            Parsed::JavaScript(parse, source_type) => {
                Some(FormatNode::JavaScript(parse.syntax(), *source_type))
            }
            Parsed::Json(parse) => Some(FormatNode::Json(parse.syntax())),
        }
    }

    pub fn analyze(&self) -> Option<Analyze> {
        match self {
            Parsed::JavaScript(parse, _) => Some(Analyze::JavaScript(parse.tree())),
            Parsed::Json(_) => None,
        }
    }

    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        match self {
            Parsed::JavaScript(parse, _) => parse.into_diagnostics(),
            Parsed::Json(parse) => parse.into_diagnostics(),
        }
    }
}

pub enum FormatNode {
    JavaScript(JsSyntaxNode, JsFileSource),
    Json(JsonSyntaxNode),
}

impl FormatNode {
    pub fn format_node(&self) -> FormatResult<FormattedNode> {
        match self {
            Self::JavaScript(root, source_type) => {
                tuna_js_formatter::format_node(JsFormatOptions::new(*source_type), root)
                    .map(FormattedNode::JavaScript)
            }
            FormatNode::Json(root) => {
                tuna_json_formatter::format_node(JsonFormatOptions::default(), root)
                    .map(FormattedNode::Json)
            }
        }
    }
}

pub enum FormattedNode {
    JavaScript(Formatted<JsFormatContext>),
    Json(Formatted<JsonFormatContext>),
}

impl FormattedNode {
    pub fn print(&self) -> PrintResult<Printed> {
        match self {
            FormattedNode::JavaScript(formatted) => formatted.print(),
            FormattedNode::Json(formatted) => formatted.print(),
        }
    }
}

pub enum Analyze {
    JavaScript(AnyJsRoot),
}

impl Analyze {
    pub fn analyze(&self) {
        match self {
            Analyze::JavaScript(root) => {
                let filter = AnalysisFilter {
                    categories: RuleCategories::SYNTAX | RuleCategories::LINT,
                    ..AnalysisFilter::default()
                };
                let options = AnalyzerOptions::default();
                analyze(root, filter, &options, JsFileSource::default(), |event| {
                    black_box(event.diagnostic());
                    black_box(event.actions());
                    ControlFlow::<Never>::Continue(())
                });
            }
        }
    }
}
