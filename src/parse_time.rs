pub fn parse_for_wait_time(args: Vec<&str>) -> (String, i32, i32) {
    let mut reply_msg = String::from("Failed to parse date.");
    let mut time_to_wait_in_seconds: i32 = 0;
    let mut used_args = 0;
    if args.len() >= 2 {
        used_args = 2;
        time_to_wait_in_seconds = match args[0].parse::<i32>() {
            Ok(n) => match args[1].as_ref() {
                "s" | "second" | "seconds" | "sec" | "secs" => {
                    reply_msg = format!("{} seconds", n);
                    n
                }
                "m" | "minute" | "minutes" | "min" | "mins" => {
                    reply_msg = format!("{} minutes", n);
                    n * 60
                }
                "h" | "hour" | "hours" | "hr" | "hrs" => {
                    reply_msg = format!("{} hours", n);
                    n * 60 * 60
                }
                "d" | "day" | "days" => {
                    reply_msg = format!("{} days", n);
                    n * 60 * 60 * 24
                }
                "w" | "week" | "weeks" => {
                    reply_msg = format!("{} days", n);
                    n * 60 * 60 * 24 * 7
                }
                "month" | "months" => {
                    reply_msg = format!("{} days", n);
                    n * 60 * 60 * 24 * 7 * 4
                }
                "y" | "year" | "years" => {
                    reply_msg = format!("{} days", n);
                    n * 60 * 60 * 24 * 7 * 4 * 12
                }
                _ => {
                    reply_msg = format!("{} minutes", n);
                    n * 60
                }
            },
            Err(e) => 0,
        };
    }

    (reply_msg, time_to_wait_in_seconds, used_args)
}
