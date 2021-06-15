use serenity::{
    model::{channel::Message},
    prelude::*,
};

pub async fn handle_message(ctx: Context, msg: Message) {
    if msg.content == "!ping" {
        // Sending a message can fail, due to a network error, an
        // authentication error, or lack of permissions to post in the
        // channel, so log to stdout when some error happens, with a
        // description of it.
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
            println!("Error sending message: {:?}", why);
        }
    }
}

// fn handle_help(msg: Message) {

// }
