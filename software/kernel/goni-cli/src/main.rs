use std::sync::Arc;

use futures_util::StreamExt;
use goni_context::{FacilityLocationSelector, NullKvPager};
use goni_core::GoniKernel;
use goni_infer::NullLlmEngine;
use goni_router::NullRouter;
use goni_sched::InMemoryScheduler;
use goni_store::NullDataPlane;
use goni_types::TaskClass;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    Ok(())
}
