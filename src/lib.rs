#![no_std]
use dmail_mail_io::*;
use gstd::{exec, msg, Reservations};
const GAS_FOR_UPDATE: u64 = 60_000_000_000;
static mut RESERVATIONS: Reservations = Reservations::new();

#[no_mangle]
extern "C" fn handle() {
    let action: DmailAction = msg::load().expect("Could not load `DmailAction`");
    match action {
        DmailAction::Reserve { amount, duration } => unsafe {
            RESERVATIONS.reserve(amount, duration).expect("Failed to reserve gas");
        },

        DmailAction::SendMail { to, path } => {
            if exec::gas_available() <= GAS_FOR_UPDATE {
                let reservation = unsafe { RESERVATIONS.try_take_reservation(100_000) };
                if let Some(reservation) = reservation {
                    msg::send_from_reservation(reservation.id(), exec::program_id(), DmailAction::SendMail { to: to.clone(), path: path.clone() }, 0).expect("Failed to send message");
                } else {
                    panic!("Reservation not found");
                }
            }
            msg::reply(DmailEvent::SendMail { from: msg::source(), to, path }, 0).expect("Failed to send mail");
        }
    }
}
