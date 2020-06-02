use serenity::{
    model::{
        channel::{Message, Reaction, ReactionType},
        gateway::Ready,
    },
    prelude::{Context, EventHandler},
};

pub struct Handler;

fn split_once(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, ':');
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

impl EventHandler for Handler {
    fn reaction_add(&self, ctx: Context, mut reaction: Reaction) {
        let reaction_msg = reaction.message(&ctx.http).unwrap();
        match &reaction.emoji {
            ReactionType::Unicode(uni) => match uni.as_ref() {
                "👀" => {
                    let message_content = &reaction_msg.content;
                    // let msg_args: Vec<&str> = message_content.split_whitespace().collect();
                    let mut msg_url = String::from("Url not found");
                    println!(
                        "Msg author {} reaction user {}",
                        reaction_msg.author.id, reaction.user_id
                    );

                    if reaction_msg.author.id == reaction.user_id
                        || reaction.user(&ctx).unwrap().bot
                    {
                        println!("Same user slash bot cannot use :eyes: emoji.");
                    } else {
                        println!("Start processing :eyes: emoji.");

                        if reaction_msg.is_private() {
                            msg_url = format!(
                                "http://discordapp.com/channels/@me/{}/{}",
                                reaction_msg.channel_id, reaction_msg.id
                            );
                        } else {
                            msg_url = format!(
                                "http://discordapp.com/channels/{}/{}/{}",
                                reaction_msg.guild_id.unwrap(),
                                reaction_msg.channel_id,
                                reaction_msg.id
                            );
                        }
                        // let remind_msg = format!("Reminder: \"{}\" \nLink: {}", args.rest(), &msg_url);
                        let remind_msg = format!("Reminder for link: {}", &msg_url);
                        println!("Requested reminder through :eyes: emoji.");
                        let dm = &reaction
                            .user(&ctx)
                            .unwrap()
                            .direct_message(&ctx.http, |m| m.content(remind_msg));
                    }
                    ()
                }
                _ => (),
            },
            _ => (),
        };
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
