use serenity::{
    model::{
        channel::{Message, Reaction, ReactionType},
        gateway::Ready,
    },
    prelude::{Context, EventHandler},
};

pub struct Handler;

impl EventHandler for Handler {
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Err(why) = reaction.channel_id.say(
            &ctx.http,
            format!(
                "{} left a {} reaction ",
                reaction.user(&ctx).unwrap().name,
                match reaction.emoji {
                    ReactionType::Custom {
                        animated: _animated,
                        id: _id,
                        name,
                    } => name.unwrap(),
                    ReactionType::Unicode(uni) => uni,
                    ReactionType::__Nonexhaustive => String::from("Unknown"),
                }
            ),
        ) {
            println!("Error reacting to a reaction: {:?}", why);
        }
    }
    fn message(&self, ctx: Context, _new_message: Message) {
        if _new_message.content == "???" {
            use std::thread;
            thread::sleep(std::time::Duration::new(1, 0));

            let remind_msg = format!(
                "<@{}> wants to be reminded of something.",
                &_new_message.author.id
            );
            if let Err(err) = _new_message.channel_id.say(&ctx.http, remind_msg) {
                println!("Error giving message: {:?}", err);
            }
        }
    }
    fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}
