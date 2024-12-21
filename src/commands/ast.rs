use clap::Parser;
use serde_json::json;
use termtree::Tree;

use mago_ast::node::NodeKind;
use mago_ast::Node;
use mago_interner::ThreadedInterner;
use mago_reporting::reporter::Reporter;
use mago_reporting::reporter::ReportingFormat;
use mago_reporting::reporter::ReportingTarget;
use mago_reporting::Issue;
use mago_source::SourceManager;

use crate::enum_variants;
use crate::service::ast::AstService;
use crate::utils::bail;

#[derive(Parser, Debug)]
#[command(
    name = "ast",
    about = "Prints the abstract syntax tree of a PHP file.",
    long_about = "Given a PHP file, this command will parse the file and print the abstract syntax tree (AST) to the console."
)]
pub struct AstCommand {
    #[arg(long, short = 'f', help = "The PHP file to parse.", required = true)]
    pub file: String,

    #[arg(long, help = "Outputs the result in JSON format.")]
    pub json: bool,

    #[arg(long, default_value_t, help = "The issue reporting target to use.", ignore_case = true, value_parser = enum_variants!(ReportingTarget))]
    pub reporting_target: ReportingTarget,

    #[arg(long, default_value_t, help = "The issue reporting format to use.", ignore_case = true, value_parser = enum_variants!(ReportingFormat))]
    pub reporting_format: ReportingFormat,
}

pub async fn execute(command: AstCommand) -> i32 {
    let file_path = std::path::Path::new(&command.file).to_path_buf();

    // Check if the file exists and is readable
    if !file_path.exists() {
        mago_feedback::error!("file '{}' does not exist.", command.file);

        return 1;
    }

    if !file_path.is_file() {
        mago_feedback::error!("'{}' is not a valid file.", command.file);

        return 1;
    }

    let interner = ThreadedInterner::new();
    let source_manager = SourceManager::new(interner.clone());

    let source_id = source_manager.insert_path(command.file, file_path, true);

    let service = AstService::new(interner.clone(), source_manager.clone());

    let (ast, error) = service.parse(source_id).await.unwrap_or_else(bail);

    let has_error = error.is_some();
    if command.json {
        // Prepare JSON output
        let result = json!({
            "interner": interner.all().into_iter().collect::<Vec<_>>(),
            "program": ast,
            "error": error.map(|e| Into::<Issue>::into(&e)),
        });

        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        // Print the AST as a tree
        let tree = node_to_tree(Node::Program(&ast));

        println!("{tree}");

        if let Some(error) = &error {
            let issue = Into::<Issue>::into(error);

            Reporter::new(interner, source_manager, command.reporting_target)
                .report([issue], command.reporting_format)
                .unwrap_or_else(bail);
        }
    }

    if has_error {
        1
    } else {
        0
    }
}

fn node_to_tree(node: Node<'_>) -> Tree<NodeKind> {
    let mut tree = Tree::new(node.kind());
    for child in node.children() {
        tree.push(node_to_tree(child));
    }

    tree
}
