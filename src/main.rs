use colored::{Color, Colorize};
use gptcli::Role;
use home::home_dir;
use std::io::Write;

fn get_config_path() -> String {
    let homepath = home_dir().unwrap();
    homepath
        .join(".gptcli.toml")
        .to_str()
        .unwrap()
        .to_string()
}

fn main() {
    let message = "Prompt => ".color(Color::BrightBlue);
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    // Read user input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Remove trailing newline
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

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

#[cfg(test)]
mod tests {
    use super::get_config_path;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn config_path_uses_home_directory() {
        let original = env::var("HOME").ok();
        let temp_home = env::temp_dir().join("gptcli_test_home");
        env::set_var("HOME", &temp_home);

        let path = get_config_path();
        assert_eq!(PathBuf::from(path), temp_home.join(".gptcli.toml"));

        if let Some(val) = original {
            env::set_var("HOME", val);
        } else {
            env::remove_var("HOME");
        }
    }
}
