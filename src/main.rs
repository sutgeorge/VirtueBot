use std::env;
use std::fs;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

fn ping(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
        println!("Error sending message: {:?}", why);
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => ping(&ctx, &msg),
            _ => {}
        }
    }

    fn ready(&self, __: Context, ready: Ready) {
        println!("{} has connected!", ready.user.name);
    }
}

fn main() {
    let token = fs::read_to_string("TOKEN.txt").expect("Something went wrong reading the file");
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    if let Err(msg) = client.start() {
        println!("Error: {:?}", msg);
    }
}