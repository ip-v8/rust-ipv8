mod defaultheader;

extern crate macros;
use macros::{generate_parse_header, create_header_u16};

create_header_u16!(
  0002,

  pub struct DefaultHeader{
    pub version : u16,
    pub mid_hash : [u8; 20],
    pub message_type : u8,
  }
);

//____               _ _
//|  _ \  ___  _ __ ( ) |_   _ __ ___   _____   _____
//| | | |/ _ \| '_ \|/| __| | '_ ` _ \ / _ \ \ / / _ \
//| |_| | (_) | | | | | |_  | | | | | | (_) \ V /  __/
//|____/ \___/|_| |_|  \__| |_| |_| |_|\___/ \_/ \___|
//
// (Here be dragons)
//
// anything that is below here, is here to ensure it's evaluated last.

#[generate_parse_header]
pub fn parse_header<T>(magic : u32) -> Option<T>{
//  let mut res = match magic & 0xffffffffu32{
////    42424203u32 => Some(T2::Default()),
//    // insertion point for 32 bit entries
//    _ => None
//  };
//
//  if res.is_none(){
//    res = match ((magic & 0xffff0000) >> 16) as u16{
//      0002u16 => Some(DefaultHeader::Default()),
//      // insertion point for 16 bit entries
//      _ => None
//    };
//  }
//
//  return res;
}
