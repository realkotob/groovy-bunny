#[allow(unused_parens)]
use chrono::Utc;
use serenity::model::id::ChannelId;
use std::io::Error;
extern crate job_scheduler;
use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

use serenity::prelude::Context;

pub fn schedule_announcements(ctx: &Context) -> Result<(), Error> {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("0 0 9 * * FRI".parse().unwrap(), || {
        println!("check_work_log ...");
        match check_work_log(&ctx) {
            Ok(x) => println!("Checked worklog loaded."),
            Err(why) => println!("Error checking worklog {:?}", why),
        };
    }));

    sched.add(Job::new("0 0 13 * * TUE".parse().unwrap(), || {
        println!("send_qa_day_dev_reminder ...");
        match send_qa_day_dev_reminder(&ctx) {
            Ok(x) => println!("Sent QA day dev reminder."),
            Err(why) => println!("Error sending QA day dev reminder {:?}", why),
        };
    }));

    sched.add(Job::new("0 0 8 * * WED".parse().unwrap(), || {
        println!("send_qa_day_all_reminder ...");
        match send_qa_day_all_reminder(&ctx) {
            Ok(x) => println!("Sent QA dev reminder."),
            Err(why) => println!("Error sending QA day reminder {:?}", why),
        };
    }));

    println!("Scheduled announcement, now entering scheduler loop ...");

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

pub fn send_qa_day_dev_reminder(ctx: &Context) -> Result<(), Error> {
    let dev_reminder_channel_id: u64 = 705037778471223339;

    let dev_reminder_chan = ctx.http.get_channel(dev_reminder_channel_id);
    let dev_role_id: u64 = 705034249652273153;

    let msg_dev_remider = format!(
        "Tomorrow is QA Day! Get your test builds ready <@&{}> !",
        dev_role_id.to_string()
    );

    match dev_reminder_chan {
        Ok(x) => {
            match x.guild() {
                Some(guild_lock) => {
                    guild_lock.read().say(&ctx.http, msg_dev_remider);
                }
                None => {
                    println!("It's not a guild!");
                }
            };
        }
        Err(why) => {
            println!("Error sending message to dev reminder channel. {:?}", why);
        }
    };

    Ok(())
}

pub fn send_qa_day_all_reminder(ctx: &Context) -> Result<(), Error> {
    let dev_reminder_channel_id: u64 = 705090277794119790;

    let dev_reminder_chan = ctx.http.get_channel(dev_reminder_channel_id);

    let msg_dev_remider = format!("Today is QA Day! Happy testing @here !",);

    match dev_reminder_chan {
        Ok(x) => {
            match x.guild() {
                Some(guild_lock) => {
                    guild_lock.read().say(&ctx.http, msg_dev_remider);
                }
                None => {
                    println!("It's not a guild!");
                }
            };
        }
        Err(why) => {
            println!("Error sending message to dev reminder channel. {:?}", why);
        }
    };

    Ok(())
}

pub fn check_work_log(ctx: &Context) -> Result<(), Error> {
    let worklog_channel_id: u64 = 705067423530745957;
    // let test_channel_id: u64 = 770656917415788545;

    let channel_id = ChannelId(worklog_channel_id);

    let all_devs: Vec<u64> = vec![
        492385983833047051,
        503494040436604930,
        447503701733539845,
        305360713893937162,
        669148598193225739,
    ]; // sam 669148598193225739
    let mut did_speak = vec![];
    let mut didnt_speak = vec![];
    let _messages = channel_id.messages(&ctx.http, |retriever| retriever);
    let mut didnt_size: u64 = 0;
    let mut did_size: u64 = 0;
    match _messages {
        Ok(msgs) => {
            for dev in all_devs {
                let mut dev_spoke = false;
                let mut words_count = 0;
                for elem in &msgs {
                    if (
                        (&dev == elem.author.id.as_u64())
                            && ((Utc::now().timestamp() - elem.timestamp.timestamp()) < 432000)
                        // week 604800
                    ) {
                        let mut words = elem.content.split_whitespace();
                        let local_words_count = words.count();
                        if (local_words_count > 2) {
                            words_count += local_words_count;
                        }
                    };
                }
                if (words_count > 5) {
                    dev_spoke = true;
                }
                if dev_spoke {
                    did_speak.push(dev);
                } else {
                    didnt_speak.push(dev);
                }
            }
            didnt_size = didnt_speak.len() as u64;
            did_size = did_speak.len() as u64;
        }
        Err(why) => {
            println!("Failed to get messages. {:?}", why);
        }
    }

    let work_log_channel = ctx.http.get_channel(worklog_channel_id);
    let mut msg_didnt_worklog = " remember to post your weekly progress!".to_string();
    let mut msg_did_worklog = " posted in the past week, congrats!".to_string();
    for elem in didnt_speak {
        let str_mention = format!(" <@{}>", &elem.to_string());
        msg_didnt_worklog = format!("{}{}", &str_mention, &msg_didnt_worklog).to_owned();
    }
    for elem in did_speak {
        let str_mention = format!(" <@{}>", &elem.to_string());
        msg_did_worklog = format!("{}{}", &str_mention, &msg_did_worklog);
    }
    match work_log_channel {
        Ok(x) => {
            match x.guild() {
                Some(guild_lock) => {
                    if (didnt_size > 0) {
                        guild_lock.read().say(&ctx.http, msg_didnt_worklog);
                    }
                    if (did_size > 0) {
                        guild_lock.read().say(&ctx.http, msg_did_worklog);
                    }
                }
                None => {
                    println!("It's not a guild!");
                }
            };
        }
        Err(why) => {
            println!("Error sending message to worklog channel. {:?}", why);
        }
    };

    Ok(())
}
