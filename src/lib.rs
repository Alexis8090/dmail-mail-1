#![no_std]

use dmail_mail_io::*;
use gstd::{exec, msg::{self, send_delayed}, ReservationId};

const RESERVATION_AMOUNT: u64 = 245_000_000_000;
const GAS_FOR_UPDATE: u64 = 60_000_000_000;
static mut RESERVATION: Vec<ReservationId> = vec![];

#[no_mangle]
extern "C" fn handle() {
    let action: DmailAction = msg::load().expect("Could not load `DmailAction`");
    match action {
        DmailAction::ReserveGas => {
            let reservations: &mut Vec<ReservationId> = unsafe { RESERVATION.as_mut() };
            let reservation_id = ReservationId::reserve(RESERVATION_AMOUNT, 600)
                .expect("reservation across executions");
            reservations.push(reservation_id);
            msg::reply(DmailEvent::GasReserved, 0).expect("")

        DmailAction::SendMail { to, path } => {
            if  exec::gas_available() <= GAS_FOR_UPDATE {
                let reservations: &mut Vec<ReservationId> = unsafe { RESERVATION.as_mut() };
                let reservation_id = reservations.pop().expect("Need more gas");
                msg::send_from_reservation(
                            id,
                            exec::program_id(),
                            DmailAction::SendMail{to,path},
                            0,
                        )
                        .expect("Failed to send message");
            }
            msg::reply(
                DmailEvent::SendMail {
                    from: msg::source(),
                    to,
                    path,
                },
                0,
            )
        }
    }
    .expect("Error in sending a reply to monopoly contract");
}
