use rnix::SyntaxNode;
use rnix::tokenizer;
use rnix::parser::parse;
use rnix::ast::{AstNode, Expr, AttrSet, Lambda, Apply}; // AstNode is the trait, Expr, AttrSet, Lambda, Apply are structs/enums
use rnix::ast::HasEntry; // HasEntry is a trait
use rnix::ast::Import; // Import is a struct

fn main() {
    let nix_code = "{ a = 1; b = x: x + 1; c = import ./foo.nix; d = f 1; }";
    let tokens = tokenizer::tokenize(nix_code);
    let (green_node, errors) = parse(tokens.into_iter());

    if !errors.is_empty() {
        eprintln!("Parsing errors: {:?}", errors);
    }

    let root: SyntaxNode = SyntaxNode::new_root(green_node);

    for child in root.children() {
        // Test Expr::cast
        if let Some(expr) = Expr::cast(child.clone()) {
            match expr {
                Expr::AttrSet(attr_set) => {
                    // Test HasEntry
                    println!("AttrSet entries: {}", attr_set.entries().count());
                }
                Expr::Lambda(lambda) => {
                    // Test HasParameters - I'll need to find where HasParameters is
                    // For now, I'll comment it out or use a placeholder
                    // println!("Lambda params: {}", lambda.param().is_some() as usize);
                }
                Expr::Apply(apply) => {
                    // Test Apply::function / callee
                    if let Some(func_expr) = apply.lambda().and_then(|f| f.cast::<Expr>()) {
                        println!("Apply function: {:?}", func_expr.syntax().text());
                    }
                }
                _ => {}
            }
        }
        // Test Import::cast
        if let Some(import_node) = Import::cast(child.clone()) {
            println!("Import path: {:?}", import_node.path());
        }
    }
}
