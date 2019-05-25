// module attempting to ensure feature parity with py-ipv8/ipv8/messaging/payload.py
pub mod connectiontype;
pub mod introductionrequestpayload;
pub mod puncturepayload;
pub mod puncturerequestpayload;
pub mod timedistributionpayload;
pub mod binmemberauthenticationpayload;

// used to recognize payloads. Does not have any members *yet*
pub trait Ipv8Payload {
}
