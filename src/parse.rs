use mailparse::{parse_mail, MailHeaderMap, MailParseError};

#[derive(Debug)]
pub struct MailContent {
    sender: String,
    subject: Option<String>,
    body: String,
}

type ParsedResult = Result<MailContent, MailParseError>;

pub fn parse_fetched_mail(message: String) -> ParsedResult {
    let parsed = parse_mail(message.as_bytes())?;
    let content = MailContent {
        sender: parsed.headers.get_first_value("From").unwrap(),
        subject: parsed.headers.get_first_value("Subject"),
        body: if parsed.subparts.len() == 0 {
            parsed.get_body()?
        } else {
            parsed.subparts[0].get_body()?
        },
    };

    Ok(content)
}
