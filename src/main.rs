mod text_treatment;
mod fetch;
mod imap;

use crate::imap::imap;
use crate::fetch::send_message;

fn main() {
    let resp = imap();

    send_message(resp);
}