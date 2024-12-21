use clap::Parser;

use mago_interner::ThreadedInterner;

use crate::config::Configuration;
use crate::service::formatter::FormatterService;
use crate::service::source::SourceService;
use crate::utils::bail;

#[derive(Parser, Debug)]
#[command(
    name = "format",
    aliases = ["fmt"],
    about = "Format source files",
    long_about = r#"
Format source files.

This command will format source files according to the rules defined in the configuration file.
"#
)]
pub struct FormatCommand {
    #[arg(long, short = 'w', help = "The width of the printed source code", value_name = "WIDTH")]
    pub print_width: Option<usize>,
    #[arg(long, short = 'd', help = "Run the command without writing any changes to disk")]
    pub dry_run: bool,
}

pub async fn execute(command: FormatCommand, mut configuration: Configuration) -> i32 {
    let interner = ThreadedInterner::new();

    let source_service = SourceService::new(interner.clone(), configuration.source);
    let source_manager = source_service.load().await.unwrap_or_else(bail);

    if let Some(width) = command.print_width {
        configuration.format.print_width = Some(width);
    }

    let service = FormatterService::new(configuration.format, interner.clone(), source_manager.clone());

    let changed = service.run(command.dry_run).await.unwrap_or_else(bail);

    if changed == 0 {
        mago_feedback::info!("All source files are already formatted");

        return 0;
    }

    if command.dry_run {
        mago_feedback::info!("Found {} source files that need formatting", changed);

        1
    } else {
        mago_feedback::info!("Formatted {} source files successfully", changed);

        0
    }
}
