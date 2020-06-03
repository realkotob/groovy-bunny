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

More syntax options like "tomorrow at 12" or "next week at 4pm" is planned.

## Development

- Install Rust tools
- Add your bot token to a `.token` file placed in the project directory
- Run `cargo run` in the project directory

