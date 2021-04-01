mod text_treatment;
mod fetch;
mod imap;

use crate::imap::imap;
use crate::fetch::send_message;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let resp = imap();

    println!("{:?}", resp);

    let res = send_message(resp);

    Ok(())
}