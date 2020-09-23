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
use rand::Rng;

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

fn create_object_from_json_file() -> serde_json::Value {
    let file_contents = read_file("quotes.json");
    let json_object: serde_json::Value = serde_json::from_str(file_contents.as_str()).unwrap();

    json_object
}

fn send_random_quote(ctx: &Context, msg: &Message) {
    let quotes: serde_json::Value = create_object_from_json_file();
    let mut random_number_generator = rand::thread_rng();
    let random_quote_index = random_number_generator.gen_range(0, 404);
    let quote = quotes["quotes"][random_quote_index].get("quote").unwrap();
    let author = quotes["quotes"][random_quote_index].get("author").unwrap();
    let source = quotes["quotes"][random_quote_index].get("source").unwrap();

    let formatted_quote = format!("```{}\n{}, {}```", quote,
    author.to_string().replace("\"", ""),
    source.to_string().replace("\"", ""));

    if let Err(why) = msg.channel_id.say(&ctx.http, formatted_quote) {
        println!("Error sending quote: {:?}", why);
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => ping(&ctx, &msg),
            "!quote" => send_random_quote(&ctx, &msg),
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