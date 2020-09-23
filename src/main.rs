use std::env;
use std::fs;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
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