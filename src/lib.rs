#![no_std]

use dmail_mail_io::*;
use gstd::msg;

#[no_mangle]
extern "C" fn handle() {
    let action: SendMail = msg::load().expect("Failed to decode input message");
    msg::reply(
        DmailEvent::SendMail {
            from: msg::source(),
            to: action.to,
            path: action.path,
        },
        0,
    )
    .expect("Error in sending a reply to monopoly contract");
}
