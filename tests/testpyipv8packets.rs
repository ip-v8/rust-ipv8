//use ipv8::networking::serialization::Packet;
//use ipv8::networking::payloads::binmemberauthenticationpayload::BinMemberAuthenticationPayload;
//use ipv8::networking::serialization::varlen::VarLen16;
//use ipv8::networking::payloads::timedistributionpayload::TimeDistributionPayload;
//use ipv8::networking::payloads::puncturepayload::PuncturePayload;
//use ipv8::networking::address::Address;
//use std::net::Ipv4Addr;
//use ipv8::networking::payloads::introductionrequestpayload::IntroductionRequestPayload;
//use ipv8::networking::payloads::connectiontype::ConnectionType;
//use ipv8::networking::serialization::rawend::RawEnd;
//
//// NOTE: headers are not yet supported so are excluded from all of the testcases below. Headers should be (hex) 00 02 + a hash of 20 bytes
//
//#[test]
//fn test_pyipv8_packet_noheader_notrailer_1() {
//  // this is slightly sanitized but REAL data from py-ipv8.
//  // the sanitation process was removing the headers and converting to hexadecimal.
//  // some more even more realistic tests will be coming.
//
//  let data = Packet(vec![0x00, 0x4a, 0x4c, 0x69, 0x62, 0x4e, 0x61, 0x43, 0x4c, 0x50, 0x4b, 0x3a, 0x51, 0xe7, 0x12, 0xc4, 0xeb, 0x8a, 0xc2, 0x5a, 0xe3, 0xa5, 0x68, 0x24, 0x08, 0xb2, 0xad, 0xbd, 0x6b, 0x78, 0xa4, 0x25, 0x54, 0x7f, 0x26, 0x85, 0xcf, 0xdf, 0x1e, 0xe9, 0x27, 0x0c, 0xbe, 0x7e, 0xc3, 0x36, 0xc4, 0x16, 0x0f, 0xf5, 0x72, 0x05, 0x4c, 0x87, 0x78, 0x42, 0xbe, 0x37, 0x73, 0x50, 0x45, 0xa9, 0x3b, 0xc4, 0xe2, 0x04, 0x15, 0x31, 0x6f, 0xdb, 0x14, 0x71, 0x61, 0xa2, 0xd7, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xc0, 0xa8, 0x01, 0x4b, 0x1f, 0x9a, 0x49, 0x91, 0x8a, 0xe9, 0x1e, 0x4f, 0xff, 0xa6, 0x68, ]);
//  let mut iter = data.deserialize_multiple();
//
//  assert_eq!(
//    BinMemberAuthenticationPayload {
//      public_key_bin: VarLen16(
//        vec![76, 105, 98, 78, 97, 67, 76, 80, 75, 58, 81, 231, 18, 196, 235, 138, 194, 90, 227, 165, 104, 36, 8, 178, 173, 189, 107, 120, 164, 37, 84, 127, 38, 133, 207, 223, 30, 233, 39, 12, 190, 126, 195, 54, 196, 22, 15, 245, 114, 5, 76, 135, 120, 66, 190, 55, 115, 80, 69, 169, 59, 196, 226, 4, 21, 49, 111, 219, 20, 113, 97, 162, 215, 70]
//      )
//    },
//    iter.next().unwrap()
//  );
//
//  assert_eq!(
//    TimeDistributionPayload{
//      global_time:15,
//    },
//    iter.next().unwrap()
//  );
//
//  assert_eq!(
//    PuncturePayload {
//      lan_walker_address: Address {
//        address: Ipv4Addr::new(192, 168, 1, 75),
//        port: 8090
//      },
//      wan_walker_address: Address {
//        address: Ipv4Addr::new(73,145,138,233),
//        port: 7759
//      },
//      identifier: 65446
//    },
//    iter.next().unwrap()
//  )
//}
//
//#[test]
//fn test_pyipv8_packet_noheader_notrailer_2() {
//  // this is slightly sanitized but REAL data from py-ipv8.
//  // the sanitation process was removing the headers and converting to hexadecimal.
//  // some more even more realistic tests will be coming.
//
//  let data = Packet(vec![0x00, 0x4a, 0x4c, 0x69, 0x62, 0x4e, 0x61, 0x43, 0x4c, 0x50, 0x4b, 0x3a, 0x51, 0xe7, 0x12, 0xc4, 0xeb, 0x8a, 0xc2, 0x5a, 0xe3, 0xa5, 0x68, 0x24, 0x08, 0xb2, 0xad, 0xbd, 0x6b, 0x78, 0xa4, 0x25, 0x54, 0x7f, 0x26, 0x85, 0xcf, 0xdf, 0x1e, 0xe9, 0x27, 0x0c, 0xbe, 0x7e, 0xc3, 0x36, 0xc4, 0x16, 0x0f, 0xf5, 0x72, 0x05, 0x4c, 0x87, 0x78, 0x42, 0xbe, 0x37, 0x73, 0x50, 0x45, 0xa9, 0x3b, 0xc4, 0xe2, 0x04, 0x15, 0x31, 0x6f, 0xdb, 0x14, 0x71, 0x61, 0xa2, 0xd7, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xaf, 0xc0, 0xa8, 0x01, 0x4c, 0x1f, 0x9a, 0x51, 0xab, 0x1b, 0xc2, 0x3d, 0xf7, 0x79, 0x93]);
//  let mut iter = data.deserialize_multiple();
//
//  assert_eq!(
//    BinMemberAuthenticationPayload {
//      public_key_bin: VarLen16(
//        vec![76, 105, 98, 78, 97, 67, 76, 80, 75, 58, 81, 231, 18, 196, 235, 138, 194, 90, 227, 165, 104, 36, 8, 178, 173, 189, 107, 120, 164, 37, 84, 127, 38, 133, 207, 223, 30, 233, 39, 12, 190, 126, 195, 54, 196, 22, 15, 245, 114, 5, 76, 135, 120, 66, 190, 55, 115, 80, 69, 169, 59, 196, 226, 4, 21, 49, 111, 219, 20, 113, 97, 162, 215, 70]
//      )
//    },
//    iter.next().unwrap()
//  );
//
//  assert_eq!(
//    TimeDistributionPayload{
//      global_time: 431,
//    },
//    iter.next().unwrap()
//  );
//
//  assert_eq!(
//    PuncturePayload {
//      lan_walker_address: Address {
//        address: Ipv4Addr::new(192, 168, 1, 76),
//        port: 8090
//      },
//      wan_walker_address: Address {
//        address: Ipv4Addr::new(81,171,27,194),
//        port: 15863
//      },
//      identifier: 31123
//    },
//    iter.next().unwrap()
//  )
//}
//
//#[test]
//fn test_pyipv8_packet_noheader_notrailer_3() {
//  // this is slightly sanitized but REAL data from py-ipv8.
//  // the sanitation process was removing the headers and converting to hexadecimal.
//  // some more even more realistic tests will be coming.
//
//  let data = Packet(vec![0x00, 0x4a, 0x4c, 0x69, 0x62, 0x4e, 0x61, 0x43, 0x4c, 0x50, 0x4b, 0x3a, 0x51, 0xe7, 0x12, 0xc4, 0xeb, 0x8a, 0xc2, 0x5a, 0xe3, 0xa5, 0x68, 0x24, 0x08, 0xb2, 0xad, 0xbd, 0x6b, 0x78, 0xa4, 0x25, 0x54, 0x7f, 0x26, 0x85, 0xcf, 0xdf, 0x1e, 0xe9, 0x27, 0x0c, 0xbe, 0x7e, 0xc3, 0x36, 0xc4, 0x16, 0x0f, 0xf5, 0x72, 0x05, 0x4c, 0x87, 0x78, 0x42, 0xbe, 0x37, 0x73, 0x50, 0x45, 0xa9, 0x3b, 0xc4, 0xe2, 0x04, 0x15, 0x31, 0x6f, 0xdb, 0x14, 0x71, 0x61, 0xa2, 0xd7, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x97, 0x51, 0xab, 0x1b, 0xc2, 0x2b, 0x67, 0xc0, 0xa8, 0x01, 0x4b, 0x1f, 0x9a, 0xc0, 0xa8, 0x01, 0x4b, 0x1f, 0x9a, 0x01, 0x00, 0x97, 0x00, 0x00, 0x00, 0x00, 0x03, 0xa5, 0xdb, 0xf5, 0xcc, 0x02, 0x1a, 0x21, 0xe9, 0x70, 0x2d, 0x8a, 0xf2, 0x91, 0xb4, 0x62, 0x49, 0x91, 0xb2, 0x41, 0x08, 0x89, 0xe3, 0x45, 0xa1, 0x9c, 0x2f, 0xf9, 0x0b, 0x60, 0x26, 0xc9, 0x70, 0x68, 0x2a, 0xbc, 0xd1, 0x03, 0x47, 0x2d, 0xfd, 0xee, 0x19, 0xd8, 0xf9, 0x48, 0x6e, 0xbf, 0x2b, 0xfd, 0xe7, 0x0c, 0x86, 0xd7, 0xbc, 0x00, 0xa6, 0x21, 0xfe, 0x26, 0x22, 0x89, 0xda, 0x0b, ]);
//  let mut iter = data.deserialize_multiple();
//
//  assert_eq!(
//    BinMemberAuthenticationPayload {
//      public_key_bin: VarLen16(
//        vec![76, 105, 98, 78, 97, 67, 76, 80, 75, 58, 81, 231, 18, 196, 235, 138, 194, 90, 227, 165, 104, 36, 8, 178, 173, 189, 107, 120, 164, 37, 84, 127, 38, 133, 207, 223, 30, 233, 39, 12, 190, 126, 195, 54, 196, 22, 15, 245, 114, 5, 76, 135, 120, 66, 190, 55, 115, 80, 69, 169, 59, 196, 226, 4, 21, 49, 111, 219, 20, 113, 97, 162, 215, 70]
//      )
//    },
//    iter.next().unwrap()
//  );
//
//  assert_eq!(
//    TimeDistributionPayload{
//      global_time: 151,
//    },
//    iter.next().unwrap()
//  );
//
//  // tmp to verify the introduction request payload. Maybe useful for verifying the todos below.
//  // let p: IntroductionRequestPayload = iter.next().unwrap();
//  // println!("{:?}",p);
//
//  assert_eq!(
//    IntroductionRequestPayload {
//
//      destination_address: Address {
//        address: Ipv4Addr::new(81, 171, 27, 194),
//        port: 11111
//      },
//      source_lan_address: Address {
//        address: Ipv4Addr::new(192,168,1,75),
//        port: 8090
//      },
//      source_wan_address: Address {
//        address: Ipv4Addr::new(192,168,1,75),
//        port: 8090
//      },
//      // TODO i am not entirely sure of the extraction of bits is 100% right. though public does seem plausible.
//      advice: false,
//      connection_type: ConnectionType::PUBLIC,
//      identifier:151,
//      // TODO: i have not at all verified if this RawEnd is correct yet.
//      extra_bytes: RawEnd(
//        vec![0, 0, 0, 0, 3, 165, 219, 245, 204, 2, 26, 33, 233, 112, 45, 138, 242, 145, 180, 98, 73, 145, 178, 65, 8, 137, 227, 69, 161, 156, 47, 249, 11, 96, 38, 201, 112, 104, 42, 188, 209, 3, 71, 45, 253, 238, 25, 216, 249, 72, 110, 191, 43, 253, 231, 12, 134, 215, 188, 0, 166, 33, 254, 38, 34, 137, 218, 11]
//      )
//    },
//    iter.next().unwrap()
//  )
//}
