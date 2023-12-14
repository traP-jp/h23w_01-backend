use std::env;

use bot_client::*;
use ImageData::*;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("BOT_ACCESS_TOKEN").unwrap();
    let client = BotClient::new(token);
    println!(
        "{:?}",
        match client
            .get_stamp_image("00076ff8-542e-4971-bad8-c3a46bc160ce".to_string())
            .await?
        {
            Svg { .. } => "svg",
            Png { .. } => "png",
            Gif { .. } => "gif",
            Jpeg { .. } => "jpeg",
        }
    );
    Ok(())
}
