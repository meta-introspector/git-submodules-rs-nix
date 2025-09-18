use rnix::{SyntaxNode, Parse}; // Parse is the result type, not a constructor
use rnix::tokenizer; // Needed for tokenizer::tokenize
use rnix::parser::parse; // Needed for parse function
use rnix::ast::{AstNode, Expr, AttrSet, LetIn, Lambda, List, Apply}; // Common AstNodes
use rnix::ast::Import; // Import is a top-level AstNode, but might need its own import line
use rnix::ast::traits::{HasEntry, HasParameters}; // Traits are in rnix::ast::traits
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

    if !errors.is_empty() {
        eprintln!("Parsing errors found in {}: {:?}\n", file_path, errors);
    }

    let root: SyntaxNode = SyntaxNode::new_root(green_node);

    Ok(collect_metrics(&root))
}

fn collect_metrics(node: &SyntaxNode) -> NixMetrics {
    let mut metrics = NixMetrics::default();

    for child in node.children() {
        if let Some(expr) = Expr::cast(child.clone()) {
            match expr {
                Expr::AttrSet(attr_set) => {
                    metrics.num_top_level_attrs = attr_set.entries().count();
                }
                Expr::LetIn(let_in) => {
                    metrics.num_let_bindings = let_in.entries().count();
                }
                Expr::Lambda(lambda) => {
                    metrics.num_function_args = lambda.params().count();
                }
                Expr::List(list) => {
                    metrics.num_list_elements = list.items().count();
                }
                Expr::Apply(apply) => {
                    if let Some(func_expr) = apply.function().and_then(|f| f.cast::<Expr>()) {
                        if func_expr.syntax().text().contains("callPackage") {
                            metrics.num_call_packages += 1;
                        }
                    }
                }
                _ => {}
            }
        } else if let Some(import_node) = Import::cast(child.clone()) {
            metrics.num_imports += 1;
            metrics.dependencies.push(import_node.path().map_or("unknown".to_string(), |p| p.to_string()));
        }
        metrics = metrics.merge(collect_metrics(&child));
    }

    metrics
}