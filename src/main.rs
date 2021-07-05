use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id.0 == 716390085896962058 {
            if let Err(why) = msg.channel_id.say(&ctx.http, "SHUT UP POKETWO!!!!!!!!!!!11!!!!!111111!!!!").await {
                println!("trolling could not be done: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("we have started a little trolling: {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("we do a little trolling");
    let token = env::var("DISCORD_TOKEN").expect("No bot token in environment");
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("troll client could not be created");
    if let Err(why) = client.start().await {
        println!("trolling could not be done: {:?}", why);
    }
}