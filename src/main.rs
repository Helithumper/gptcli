use colored::{Color, Colorize};
use gptcli::Role;
use home::home_dir;
use std::io::Write;

fn get_config_path() -> String {
    let homepath = home_dir().unwrap();
    format!("{}/.gptcli.toml", homepath.as_path().to_str().unwrap())
}

fn main() {
    let message = "Prompt => ".color(Color::BrightBlue);
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    // Read user input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Remove trailing newline
    input.pop();

    let messages = vec![gptcli::Message {
        role: Role::User.to_string(),
        content: input,
    }];

    let config_path = get_config_path();
    let client = gptcli::Gptcli::new(config_path);
    let res = client.submit_messages(messages).unwrap();

    for choice in res.choices {
        let message = format!("ChatGPT [{}]: ", choice.index).color(Color::BrightGreen);
        println!("\n{}{}", message, choice.message.content);
    }
}
