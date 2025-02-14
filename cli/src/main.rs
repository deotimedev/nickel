//! Entry point of the program.

#[cfg(feature = "doc")]
mod doc;
#[cfg(feature = "format")]
mod format;
#[cfg(feature = "metrics")]
mod metrics;
#[cfg(feature = "repl")]
mod repl;

mod cli;
mod completions;
mod customize;
mod error;
mod eval;
mod export;
mod input;
mod pprint_ast;
mod query;
mod typecheck;

use std::process::ExitCode;

use crate::cli::{Command, Options};

fn main() -> ExitCode {
    #[cfg(feature = "metrics")]
    let metrics = metrics::Recorder::install();

    let opts = <Options as clap::Parser>::parse();

    #[cfg(feature = "metrics")]
    let report_metrics = opts.global.metrics;

    let result = match opts.command {
        Command::Eval(eval) => eval.run(opts.global),
        Command::PprintAst(pprint_ast) => pprint_ast.run(opts.global),
        Command::Export(export) => export.run(opts.global),
        Command::Query(query) => query.run(opts.global),
        Command::Typecheck(typecheck) => typecheck.run(opts.global),
        Command::GenCompletions(completions) => completions.run(opts.global),

        #[cfg(feature = "repl")]
        Command::Repl(repl) => repl.run(opts.global),

        #[cfg(feature = "doc")]
        Command::Doc(doc) => doc.run(opts.global),

        #[cfg(feature = "format")]
        Command::Format(format) => format.run(opts.global),
    };

    #[cfg(feature = "metrics")]
    if report_metrics {
        metrics.report();
    }

    if let Err(e) = result {
        e.report();
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
