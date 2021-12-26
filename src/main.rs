use std::error::Error;
use teloxide::prelude::*;

async fn parse_brackets(text: &str) -> String {
    let mut q = Vec::new();
    for c in text.chars() {
        match c {
            '(' => q.push(')'),
            '（' => q.push('）'),
            ')' | '）' => {
                q.pop();
            }
            _ => continue,
        }
    }
    String::from_iter(q)
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    teloxide::enable_logging!();
    log::info!("Starting bracket closer ...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        if let Some(t) = message.update.text() {
            if let Some(s) = Some(parse_brackets(t).await).filter(|x| !String::is_empty(x)) {
                message.answer(s).await?;
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
