#[derive(Default)]
pub struct Event {}

pub trait EventHandler {
  fn on_event(&self) -> bool;
}

#[derive(Default)]
pub struct EventGenerator {
  handlers: Vec<Box<EventHandler>>,
}

impl EventGenerator {
  pub fn new() -> Self {
    EventGenerator { handlers: vec![] }
  }

  pub fn register(&mut self, handler: Box<EventHandler>) {
    self.handlers.push(handler);
  }
}
