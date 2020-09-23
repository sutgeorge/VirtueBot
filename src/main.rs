use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serde::Deserialize;

struct Handler;

fn ping(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
        println!("Error sending message: {:?}", why);
    }
}

fn read_file(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };

    contents
}

fn create_object_from_json_file() {
    let file_contents = read_file("quotes.json");
    let json_object: serde_json::Value = serde_json::from_str(file_contents.as_str()).unwrap();

    println!("{:#?}", json_object["quotes"][0]["quote"]);
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => ping(&ctx, &msg),
            "!print_json" => create_object_from_json_file(),
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