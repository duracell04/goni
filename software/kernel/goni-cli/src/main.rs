use std::sync::Arc;

use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use goni_context::{FacilityLocationSelector, NullKvPager};
use goni_core::GoniKernel;
use goni_infer::NullLlmEngine;
use goni_receipts::verify_log;
use goni_router::NullRouter;
use goni_sched::InMemoryScheduler;
use goni_store::NullDataPlane;
use goni_types::TaskClass;

#[derive(Parser)]
#[command(name = "goni")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Demo,
    Receipts {
        #[command(subcommand)]
        action: ReceiptCommand,
        #[arg(long, default_value = "./receipts.jsonl")]
        path: String,
    },
}

#[derive(Subcommand)]
enum ReceiptCommand {
    Tail { #[arg(long, default_value_t = 10)] lines: usize },
    Verify,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Demo => {
            let kernel = GoniKernel::new(
                Arc::new(NullDataPlane),
                Arc::new(FacilityLocationSelector::new(0.3)), // gamma hyperparam
                Arc::new(NullKvPager),
                Arc::new(InMemoryScheduler::new()),
                Arc::new(NullRouter),
                Arc::new(NullLlmEngine),
            );

            let prompt = "Hello, Goni!";
            let mut stream = kernel
                .handle_user_query(prompt, TaskClass::Interactive)
                .await?;

            println!("Prompt: {prompt}");

            while let Some(tok) = stream.next().await {
                let tok = tok?;
                print!("{}", tok.text);
            }
        }
        Command::Receipts { action, path } => match action {
            ReceiptCommand::Tail { lines } => {
                let content = std::fs::read_to_string(&path)?;
                let all: Vec<&str> = content.lines().collect();
                let start = all.len().saturating_sub(lines);
                for line in &all[start..] {
                    println!("{line}");
                }
            }
            ReceiptCommand::Verify => {
                verify_log(&path)?;
                println!("receipt log ok");
            }
        },
    }

    Ok(())
}
