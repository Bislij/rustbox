use poise::CreateReply;
use std::process::Command;

use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn neofetch(ctx: Context<'_>) -> Result<(), Error> {
    let neofetch_data = Command::new("sh")
        .arg("-c")
        .arg("neofetch --stdout")
        .output()
        .expect("failed to execute process");
    let response = CreateReply::default().content(format!(
        "```{}```",
        String::from_utf8(neofetch_data.stdout).unwrap(),
    ));
    ctx.send(response).await?;
    Ok(())
}
