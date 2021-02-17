use native_tls::TlsConnector;
use quoted_printable::{ decode, ParseMode };
use dissolve::strip_html_tags;

use crate::text_treatment::text_treatment;

pub fn imap() -> std::vec::Vec<std::string::String> {
    dotenv::dotenv().ok();

    let domain =  "imap.gmail.com";
    let tls = TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    let mut imap_session = client.login(dotenv::var("MAIL").unwrap(), dotenv::var("PASSWORD").unwrap()).unwrap();

    imap_session.select("INBOX").unwrap();

    let message = imap_session.fetch("1", "RFC822").unwrap();

    let mut arr: std::vec::Vec<std::string::String> = vec!["".to_string()];

    if let Some(body) = message[0].body() { 
    
        let body_decoded = decode(body, ParseMode::Robust).unwrap();
        let body_stripped = strip_html_tags(std::str::from_utf8(&body_decoded).unwrap());
        
        arr = text_treatment(body_stripped);
    } else {
        println!("Message didn't have a body!");
    }

    imap_session.logout().unwrap();
    if arr.len() == 0 {
        panic!("problem with the imap req!");
    } else {
        arr
    }
}