use std::error::Error;
use teloxide::prelude::*;

async fn parse_brackets(text: &str) -> Option<String> {
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
    Some(q)
        .filter(|x| !x.is_empty())
        .map(|x| String::from_iter(x.iter().rev()))
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    teloxide::enable_logging!();
    log::info!("Starting bracket closer ...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        if let Some(t) = message.update.text().or_else(|| message.update.caption()) {
            if let Some(s) = parse_brackets(t).await {
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
