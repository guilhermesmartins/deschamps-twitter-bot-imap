use imap;
use native_tls;

fn main() -> imap::error::Result<Option<String>> {
    let domain = "imap.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    let mut imap_session = client
        .login("holomariaserver@gmail.com", "VVbgmineg4503698741147893")
        .map_err(|e| e.0)?;

    imap_session.select("INBOX")?;

    let messages = imap_session.fetch("1", "RC822")?;
    let message = if let Some(m) = messages
        .iter()
        .next() {
            m
        }
        else {
            return Ok(None);
        };
    
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    imap_session.logout()?;

    //Ok(Some(body))
}
