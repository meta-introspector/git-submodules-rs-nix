use rnix::{SyntaxNode, NodeOrToken, Parse};
use rnix::parser::parse;
use rnix::tokenizer;
use rnix::ast::{Root, Expr};
use rnix::ast::Expr::{AttrSet, LetIn, Lambda, List, Apply, Import};
use rowan::ast::AstNode;
use std::fs;

#[derive(Debug, Default)]
pub struct NixMetrics {
    pub num_top_level_attrs: usize,
    pub num_let_bindings: usize,
    pub num_function_args: usize,
    pub num_list_elements: usize,
    pub num_imports: usize,
    pub num_call_packages: usize,
    pub dependencies: Vec<String>,
}

impl NixMetrics {
    pub fn merge(mut self, other: NixMetrics) -> Self {
        self.num_top_level_attrs += other.num_top_level_attrs;
        self.num_let_bindings += other.num_let_bindings;
        self.num_function_args += other.num_function_args;
        self.num_list_elements += other.num_list_elements;
        self.num_imports += other.num_imports;
        self.num_call_packages += other.num_call_packages;
        self.dependencies.extend(other.dependencies);
        self
    }
}

pub fn analyze_nix_file(file_path: &str) -> Result<NixMetrics, Box<dyn std::error::Error>> {
    let nix_code = fs::read_to_string(file_path)?;

    let tokens = tokenizer::tokenize(&nix_code);

    let (green_node, errors) = parse(tokens.into_iter());

    let parse_result = Parse::new(green_node, errors);

    if !parse_result.errors().is_empty() {
        eprintln!("Parsing errors found in {}: {:?}\n", file_path, parse_result.errors());
    }

    let root: SyntaxNode = parse_result.syntax();

    Ok(collect_metrics(&root))
}

fn collect_metrics(node: &SyntaxNode) -> NixMetrics {
    let mut metrics = NixMetrics::default();

    for child in node.children() {
        match Expr::cast(child) {
            Some(Expr::AttrSet(attr_set)) => {
                metrics.num_top_level_attrs = attr_set.attrs().count();
            }
            Some(Expr::LetIn(let_in)) => {
                metrics.num_let_bindings = let_in.bindings().count();
            }
            Some(Expr::Lambda(lambda)) => {
                metrics.num_function_args = lambda.args().count();
            }
            Some(Expr::List(list)) => {
                metrics.num_list_elements = list.items().count();
            }
            Some(Expr::Apply(apply)) => {
                if let Some(func) = apply.lambda().and_then(|n| n.as_node()) {
                    if func.text().contains("callPackage") {
                        metrics.num_call_packages += 1;
                    }
                }
            }
            Some(Expr::Import(import_expr)) => {
                metrics.num_imports += 1;
                metrics.dependencies.push(import_expr.path().map_or("unknown".to_string(), |p| p.to_string()));
            }
            _ => {}
        }
        metrics = metrics.merge(collect_metrics(&child));
    }

    metrics
}