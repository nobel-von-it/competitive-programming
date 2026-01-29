pub fn format_duration_gpt_help(seconds: u64) -> String {
    if seconds == 0 {
        return "now".to_string();
    }
    let years = seconds / (365 * 24 * 3600);
    let days = (seconds % (365 * 24 * 3600)) / (24 * 3600);
    let hours = (seconds % (24 * 3600)) / (3600);
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    let mut res = Vec::new();

    if years > 0 {
        res.push(format!("{years} year{}", if years > 1 { "s" } else { "" }));
    }
    if days > 0 {
        res.push(format!("{days} day{}", if days > 1 { "s" } else { "" }));
    }
    if hours > 0 {
        res.push(format!("{hours} hour{}", if hours > 1 { "s" } else { "" }));
    }
    if minutes > 0 {
        res.push(format!(
            "{minutes} minute{}",
            if minutes > 1 { "s" } else { "" }
        ));
    }
    if seconds > 0 {
        res.push(format!(
            "{seconds} second{}",
            if seconds > 1 { "s" } else { "" }
        ));
    }
    if res.len() > 1 {
        let last2 = res.split_last().unwrap();
        format!("{} and {}", last2.1.join(", "), last2.0)
    } else {
        res.join(", ")
    }
}
