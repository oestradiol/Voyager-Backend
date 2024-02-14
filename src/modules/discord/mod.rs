use serenity::all::Webhook;
use serenity::builder::CreateEmbed;
use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use tracing::event;
use tracing::Level;

use crate::configs::environment::DEVELOPMENT;
use crate::configs::environment::DISCORD_WEBHOOK;
use crate::types::model::deployment::Deployment;
use crate::Error;

async fn send_deployment_message(deployment: &Deployment) -> Result<(), Error> {
  if *DEVELOPMENT {
    return Ok(());
  }

  event!(Level::INFO, "Sending deployment discord message for deployment");

  let mode = deployment.mode.to_string();

  let embed = CreateEmbed::new()
    .title(format!("[New {} deployment](https://{})", mode, deployment.host))
    .description(format!("A new {mode} deployment has been created."))
    .field("ID", deployment.id.clone(), true)
    .field("Docker Container", deployment.container_id.clone(), true);
  let builder = ExecuteWebhook::new().username("Voyager API").embed(embed);

  let http = Http::new("");
  let webhook_client = Webhook::from_url(&http, &DISCORD_WEBHOOK).await;
  match webhook_client {
    Ok(webhook_client) => {
      //           webhookClient.send(webhookMessage)
      //               .whenCompleteAsync { _: ReadonlyMessage?, err: Throwable? ->
      //                   err?.let { log("Error sending discord webhook message: ${err.message}", LogType.ERROR) }
      //               }
      match webhook_client.execute(&http, false, builder).await {
        Ok(msg) => {
          let message = msg.map_or(String::new(), |msg| format!(" Returned message is: {}", msg.content));
          event!(Level::INFO, "Discord webhook sent successfuly!{}", message);
        }
        Err(e) => {
          event!(Level::ERROR, "Error sending webhook discord message: {}", e);
        }
      }
    },
    Err(e) => {
      event!(Level::ERROR, "Discord webhook client was not created! {}", e);
    }
  }


  Ok(())
}
