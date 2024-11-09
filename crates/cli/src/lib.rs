use std::process::exit;

use clap::Parser;
use tokio::runtime::Builder;

use fennec_feedback::initialize_logger;
use fennec_feedback::LevelFilter;
use fennec_service::config::Configuration;

use crate::commands::FennecCommand;
use crate::utils::bail;

pub mod commands;
pub mod utils;

pub fn run() -> ! {
    // Set up the logger.
    initialize_logger(LevelFilter::WARN, "FENNEC_LOG");

    // Load the configuration.
    let configuration = Configuration::load().unwrap_or_else(bail);

    // Create the runtime.
    let runtime = Builder::new_multi_thread()
        .worker_threads(configuration.threads)
        .thread_stack_size(configuration.stack_size)
        .enable_all()
        .build()
        .unwrap_or_else(bail);

    let code = match FennecCommand::parse() {
        FennecCommand::Lint(cmd) => runtime.block_on(commands::lint::execute(cmd, configuration)),
        FennecCommand::Fix(cmd) => runtime.block_on(commands::fix::execute(cmd, configuration)),
    };

    exit(code)
}
