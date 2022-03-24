use imap::{self, types::UnsolicitedResponse};
use serde_derive::Deserialize;
use std::sync::mpsc::Sender;

// pub type ImapResult = imap::error::Result<Option<String>>;

#[derive(Deserialize, Debug)]
pub struct ImapParams {
    server: String,

    #[serde(default = "default_port")]
    port: u16,

    username: String,
    password: String,
    mailbox: String,
}

fn default_port() -> u16 {
    993
}

pub fn get_mail_with_imap(imap_params: ImapParams, tx: Sender<Option<String>>) {
    // サーバに接続
    let client = imap::ClientBuilder::new(imap_params.server.clone(), imap_params.port)
        .native_tls()
        .expect("failed to connect to server");

    // ログインセッションを作成
    let mut session = client
        .login(imap_params.username, imap_params.password)
        .expect("failed to authenticate");

    session.debug = true;

    // メールボックスを指定
    session
        .select(imap_params.mailbox)
        .expect("failed to select mailbox");

    loop {
        // 通知がくるまで待つ
        let mut count = 0;
        let idle_result = session.idle().wait_while(|res| {
            match res {
                UnsolicitedResponse::Exists(n) => {
                    count = n;
                    false
                },
                _ => true
            }
        });

        match idle_result {
            Ok(reason) => println!("IDLE finished normally {:?}", reason),
            Err(e) => {
                println!("IDLE finished with error {:?}", e);
                break;
            }
        }

        // 新メールをフェッチ
        let messages = session
            .fetch(&count.to_string(), "BODY.PEEK[]")
            .expect("failed to fetch message");
        let message = messages.iter().next().expect("no next mail");

        let body = message.body().expect("no body");
        let body = std::str::from_utf8(body)
            .expect("not valid utf-8")
            .to_string();

        println!("body: {}", body);
        tx.send(Some(body)).unwrap();
    }

    session.logout().expect("Could not log out");
}
