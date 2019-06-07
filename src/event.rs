#[derive(Default)]
pub struct Event {}

pub trait EventHandler {
  fn on_event(&self) -> bool;
}

#[derive(Default)]
pub struct EventGenerator {
  handlers: Vec<Box<dyn EventHandler>>,
}

impl EventGenerator {
  pub fn new() -> Self {
    EventGenerator { handlers: vec![] }
  }

  pub fn register(&mut self, handler: Box<dyn EventHandler>) {
    self.handlers.push(handler);
  }
}
