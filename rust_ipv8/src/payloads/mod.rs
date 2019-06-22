//! This module has all of the original py-ipv8 payloads in it to be put
//! into packets and sent in a way such that py-ipv8 can interpret what
//! we are sending.
// module attempting to ensure feature parity with py-ipv8/ipv8/messaging/payload.py
pub mod binmemberauthenticationpayload;
pub mod connectiontype;
pub mod introductionrequestpayload;
pub mod introductionresponsepayload;
pub mod puncturepayload;
pub mod puncturerequestpayload;
pub mod timedistributionpayload;

/// Used to recognize payloads. Does not have any members _yet_, (though already useful as a marker trait).
pub trait Ipv8Payload {}
