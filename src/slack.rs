use slack;
use crate::parse::MailContent;

pub fn send_to_slack(mail_content: MailContent) -> Result<(), String> {
    println!("{:?}", mail_content);
    Ok(())
}
