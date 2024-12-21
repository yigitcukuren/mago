use std::process::exit;

use clap::Parser;
use tokio::runtime::Builder;

use mago_feedback::initialize_logger;
use mago_feedback::LevelFilter;
use mago_service::config::Configuration;

use crate::commands::MagoCommand;
use crate::utils::bail;

pub mod commands;
pub mod utils;

pub fn main() -> ! {
    // Set up the logger.
    initialize_logger(if cfg!(debug_assertions) { LevelFilter::DEBUG } else { LevelFilter::INFO }, "MAGO_LOG");

    // Load the configuration.
    let configuration = Configuration::load().unwrap_or_else(bail);

    // Create the runtime.

    let runtime = if configuration.threads <= 1 {
        Builder::new_current_thread().enable_all().build().unwrap_or_else(bail)
    } else {
        Builder::new_multi_thread()
            .worker_threads(configuration.threads)
            .thread_stack_size(configuration.stack_size)
            .enable_all()
            .build()
            .unwrap_or_else(bail)
    };

    let code = match MagoCommand::parse() {
        MagoCommand::Lint(cmd) => runtime.block_on(commands::lint::execute(cmd, configuration)),
        MagoCommand::Fix(cmd) => runtime.block_on(commands::fix::execute(cmd, configuration)),
        MagoCommand::Format(cmd) => runtime.block_on(commands::format::execute(cmd, configuration)),
        MagoCommand::Ast(cmd) => runtime.block_on(commands::ast::execute(cmd)),
        MagoCommand::SelfUpdate(cmd) => commands::self_update::execute(cmd, configuration),
    };

    exit(code)
}
