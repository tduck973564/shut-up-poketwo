use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
	if msg.content == "fuck you chriswin" {
                let chriswin = serenity::model::id::UserId(811068298934091777);
		loop {
                    let response = MessageBuilder::new().mention(&chriswin).build();
		    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                        println!("Horbs");
                    }
                }
        } else if msg.author.id.0 == 716390085896962058 {
            if let Err(why) = msg.channel_id.say(&ctx.http, "SHUT UP POKETWO!!!!!!!!!!!11!!!!!111111!!!!").await {
                println!("trolling could not be done: {:?}", why);
            }
        } else if msg.author.id.0 == 275959780164632576 {
            if let Err(why) = msg.channel_id.say(&ctx.http, "SHUT UP FUST!!!!!!!!!!!11!!!!!111111!!!!").await {
                println!("trolling could not be done: {:?}", why);
            }
        } else if msg.author.id.0 == 791600981158264874 {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Ok chim").await {
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
