use native_tls::TlsConnector;
use imap::types::Fetch;
use dotenv;

fn main() {
    dotenv::dotenv().ok();

    let domain =  "imap.gmail.com";
    let tls = TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    let mut imap_session = client.login(dotenv::var("MAIL").unwrap(), dotenv::var("PASSWORD").unwrap()).unwrap();

    imap_session.select("INBOX").unwrap();

    let message = imap_session.fetch("1", "RFC822").unwrap();

    if let Some(body) = message[0].body() {
        println!("{}", std::str::from_utf8(body).unwrap());
    } else {
        println!("Message didn't have a body!");
    }

    imap_session.logout().unwrap();
}