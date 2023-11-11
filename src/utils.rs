use colored::Colorize;

const LOG_MARGIN: usize = 20;

pub fn log(action: &str, value: &str) {
    println!("{0:>LOG_MARGIN$} {1}", action.bright_green().bold(), value);
}

pub fn system_log(action: &str, value: &str) {
    println!(
        "{0:>LOG_MARGIN$} {1}\n",
        action.yellow().bold(),
        value.bright_yellow()
    );
}

pub fn blue_log(action: &str, value: &str) {
    println!(
        "{0:>LOG_MARGIN$} {1}",
        action.cyan().bold(),
        value.bright_cyan()
    );
}

pub fn black_log(action: &str, value: &str) {
    println!(
        "{0:>LOG_MARGIN$} {1}",
        action.cyan().bold(),
        value.bright_black()
    );
}
