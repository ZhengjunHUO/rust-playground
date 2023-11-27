use anyhow::Result;
use slack_morphism::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let token_raw: SlackApiTokenValue = env::var("SLACK_TOKEN")
        .expect("env var SLACK_TOKEN is required !")
        .into();
    let token: SlackApiToken = SlackApiToken::new(token_raw);

    let channel: SlackChannelId = env::var("SLACK_CHANNEL")
        .expect("env var SLACK_CHANNEL is required !")
        .into();

    let client = SlackClient::new(SlackClientHyperConnector::new());
    let s = client.open_session(&token);

    let req = SlackApiChatPostMessageRequest::new(
        channel,
        SlackMessageContent::new().with_text("Hello from rustacean !".into()),
    );

    let resp = s.chat_post_message(&req).await?;

    println!("Resp: {:?}", resp);
    Ok(())
}
