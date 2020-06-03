<!-- # White Rabbit -->
<!-- ![White Rabbit](media/sir_bunny.png? "White Rabbit logo") -->
<img src="media/sir_bunny.png" alt="White Rabbit" height="100">

<!-- [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) -->

> Oh dear! Oh dear! I shall be too late!

A bot to manage your calendar and schedule.

## Features

* **Quick Reminders** - Set a reminder using intuitive syntax e.g. `!remindme 45 minutes Call martha`
    * Sends reminders directly via DM
    * Other users on the channel can react to the message to subscribe to the reminder
    * Works both in private with the bot or inside a server
* **Google Calendar** - Work in progress

### RemindMe Syntax

> !remindme <some_integer> <time_unit_string>

e.g. !remindme 10 minutes Check the microwave

More syntax options like "tomorrow at 12" or "next week at 4pm" are planned.

## Development

Quick Start:
- Install [Rust tools](https://www.rust-lang.org/tools/install)
- Create `.token` file in the project root with the [discord bot token](https://www.writebots.com/discord-bot-token/) inside
- Run `cargo run` in the project directory

If using VSCode it is highly recommended to install the [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) and [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) plugins.

### Structure

Commands are parsed in [main.rs](src/main.rs) and other events are handled in [events.rs](src/events.rs). 

Code is refactored into separate scripts as the project grows in order to keep the main handler scripts tight and clean.

### Resources and Help

See [this video playlist](https://www.youtube.com/playlist?list=PLPwSz_Jcam3xVjrTAYgIHvf1Jq94yrRXp) for working with Discord bots in Rust.

See the [serenity docs](https://docs.rs/crate/serenity/0.8.6) for general help with the [serenity-rs](https://github.com/serenity-rs/serenity/) library.