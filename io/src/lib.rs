#![no_std]

use gmeta::{InOut, Metadata};
use gstd::prelude::*;
use gstd::{ActorId, Decode, Encode, TypeInfo};
pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<SendMail, DmailEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct SendMail {
    pub to:String,
    pub path:String,
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
}

