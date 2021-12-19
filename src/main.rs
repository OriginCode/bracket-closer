use std::error::Error;
use teloxide::prelude::*;

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    teloxide::enable_logging!();
    log::info!("Starting bracket closer ...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        if let Some(t) = message.update.text() {
            if t.ends_with("(") {
                log::info!("Someone forgot to close the \"(\" bracket!");
                message.answer(")").await?;
            } else if t.ends_with("（") {
                log::info!("Someone forgot to close the \"（\" bracket!");
                message.answer("）").await?;
            }
        }
        respond(())
    })
    .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    run().await?;
    Ok(())
}
