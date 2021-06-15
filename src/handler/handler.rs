use serenity::{
    model::{channel::Message, id::ChannelId},
    prelude::*,
};

use rand::RngCore;
use rand::rngs::OsRng;

pub async fn handle_message(ctx: Context, msg: Message) {
    let channel_id = msg.channel_id; 

    if msg.content == "!hello" {
        greet(ctx, channel_id).await;
    }
}

async fn greet(ctx: Context, channel_id: ChannelId) {
    // TODO: Separate to text file
    // TODO: Pre-fetch all greetings from the file
    let greeting_list = vec![
        "Yaa~! Have a nice day desu~",
        "Hi~!",
        "Yahalo~!",
    ];

    let random_idx = (OsRng.next_u64() as usize) % greeting_list.len();
    say(ctx, channel_id, greeting_list[random_idx]).await;
}

async fn say(ctx: Context, channel_id: ChannelId, text: &str) {
    // Sending a message can fail, due to a network error, an
    // authentication error, or lack of permissions to post in the
    // channel, so log to stdout when some error happens, with a
    // description of it.
    if let Err(why) = channel_id.say(&ctx.http, text).await {
        println!("Error sending message: {:?}", why);
    }
}
