use dotenv::dotenv;
use envy;
use imap_to_slack::{imap, parse, slack};
use std::sync::mpsc;
use std::thread;

fn process_mail(message: Option<String>) -> Result<(), String> {
    let mail_content =
        parse::parse_fetched_mail(message.unwrap()).map_err(|err| err.to_string())?;
    slack::send_to_slack(mail_content)?;
    Ok(())
}

fn main() {
    dotenv().ok();

    let imap_params = match envy::from_env::<imap::ImapParams>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        imap::get_mail_with_imap(imap_params, tx);
    });

    for message in rx {
        process_mail(message).unwrap_or_else(|err| eprintln!("Error: {}", err));
    }
}
