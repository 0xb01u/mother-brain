/*
 *  Mother Brain: Discord bot for kinda securely generating kinda secure
 *  passwords.
 *  Copyright (C) 2023-2024  Bolu <bolu@tuta.io>
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
mod commands;

use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

struct Bot;

#[serenity::async_trait]
impl EventHandler for Bot {
    // Process slash commands:
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // Authentication: only serve my user ID.
            let usr = &command.user;
            let usr_id = usr.id.get();
            let name = &usr.name;

            if usr_id != 231844961878802442 {
                let content = CreateInteractionResponseMessage::new()
                    .content(format!("I am sorry {}, I'm afraid I can't do that.", name))
                    .ephemeral(true);
                let builder = CreateInteractionResponse::Message(content);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Could not respond to slash command: {why}");
                }

                return;
            }

            let cmd_response = match command.data.name.as_str() {
                "pswd" => Some(commands::pswd::run(&command.data.options())),
                "cracktime" => Some(commands::cracktime::run(&command.data.options())),
                _ => None,
            };

            if let Some(response) = cmd_response {
                let content = CreateInteractionResponseMessage::new()
                    .content(response)
                    .ephemeral(true);
                let builder = CreateInteractionResponse::Message(content);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Could not respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        match ready.user.discriminator {
            Some(discriminator) => println!("{}#{discriminator:#?} is connected.", ready.user.name),
            None => println!("{} is connected.", ready.user.name),
        }

        // Register slash commands:
        let commands = Command::set_global_commands(
            &ctx.http,
            vec![commands::pswd::register(), commands::cracktime::register()],
        )
        .await
        .unwrap();

        println!(
            "Registered the following commands: {:?}",
            commands
                .into_iter()
                .map(|cmd| cmd.name)
                .collect::<Vec<String>>()
        );
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Erroneous DISCORD_TOKEN provided.");
    let mut client = Client::builder(&token, GatewayIntents::default())
        .event_handler(Bot)
        .await
        .expect("Could not build the Discord client.");

    client
        .start()
        .await
        .expect("The client crashed for some reason!");
}
