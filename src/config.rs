pub struct Config {
  /// Default list of host used for peer discovery
  pub default_hosts: Vec<(String, u16)>,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      default_hosts: vec![
        // Dispersy
        (String::from("1.2.3.4"), 1234),
        (String::from("130.161.119.206"), 6421),
        (String::from("130.161.119.206"), 6422),
        (String::from("131.180.27.155"), 6423),
        (String::from("131.180.27.156"), 6424),
        (String::from("131.180.27.161"), 6427),
        // IPv8
        (String::from("131.180.27.161"), 6521),
        (String::from("131.180.27.161"), 6522),
        (String::from("131.180.27.162"), 6523),
        (String::from("131.180.27.162"), 6524),
        (String::from("130.161.119.215"), 6525),
        (String::from("130.161.119.215"), 6526),
        (String::from("81.171.27.194"), 6527),
        (String::from("81.171.27.194"), 6528),
      ],
    }
  }
}
