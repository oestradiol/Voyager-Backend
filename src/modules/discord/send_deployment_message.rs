use serenity::all::Webhook;
use serenity::builder::CreateEmbed;
use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use tracing::event;
use tracing::Level;

use crate::configs::environment::DEVELOPMENT;
use crate::configs::environment::DISCORD_WEBHOOK;
use crate::types::model::deployment::Deployment;
use crate::types::model::deployment::Mode;
use crate::utils::Error;

pub async fn send_deployment_message(id: &str, name: &str, host: &str, mode: &Mode) -> Option<()> {
  if *DEVELOPMENT {
    return Some(());
  }

  event!(
    Level::INFO,
    "Sending deployment discord message for deployment"
  );

  let mode = mode.to_string();

  let embed = CreateEmbed::new()
    .title(format!("[New {mode} deployment | {name}](https://{host})",))
    .description(format!("A new {mode} deployment has been created."))
    .field("ID", id, true)
    .field("Docker Container", name, true);
  let builder = ExecuteWebhook::new().username("Voyager API").embed(embed);

  let http = Http::new("");
  let webhook_client = Webhook::from_url(&http, &DISCORD_WEBHOOK).await;
  match webhook_client {
    Ok(webhook_client) => match webhook_client.execute(&http, false, builder).await {
      Ok(msg) => {
        let message = msg.map_or(String::new(), |msg| {
          format!(" Returned message is: {}", msg.content)
        });
        event!(Level::INFO, "Discord webhook sent successfuly!{}", message);
      }
      Err(e) => {
        event!(Level::ERROR, "Error sending webhook discord message: {}", e);
      }
    },
    Err(e) => {
      event!(
        Level::ERROR,
        "Discord webhook client was not created! {}",
        e
      );
    }
  }

  Some(())
}
