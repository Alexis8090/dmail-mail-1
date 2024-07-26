#![no_std]

use gmeta::{InOut, Metadata};
use gstd::prelude::*;
use gstd::{ActorId, Decode, Encode, TypeInfo};
pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<DmailAction, DmailEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DmailAction {
    SendMail { to: String, path: String },
    ReserveGas,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DmailEvent {
    SendMail {
        from: ActorId,
        to: String,
        path: String,
    },
    GasReserved
}
