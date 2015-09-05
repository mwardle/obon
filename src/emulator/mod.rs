pub trait Interface {
  // information
  fn title(&self) -> String;
  fn videoFrequency(&self) -> f64;
  fn audioFrequency(&self) -> f64;

  // media
  fn loaded(&self) -> bool { false }
  fn sha256(&self) -> String { "".to_string() }
  fn group(&self, usize id) -> usize;
  fn load(&mut self) -> () {}
  fn save(&mut self) -> () {}
  fn unload(&mut self) -> () {}

  // system
  fn connect(&mut self, usize port, usize device) -> () {}
  fn power(&mut self) -> () {}
  fn reset(&mut self) -> () {}
  fn run(&mut self) -> () {}

  // time
  fn rtc(&self) -> bool { false }
  fn rtcsync(&self) -> () {}

  // state functions
  // TODO fn serialize(&self) -> Serializer;
  // TODO fn unserialize(&self, &Serializer serializer) -> bool;

  // cheat functions
  fn cheatSet(&self, Option<&'static str> lstring) -> () {}

  // utility functions
  // TODO

  // debugger functions
  fn tracerEnable(&self bool) -> bool { false }
  fn exportMemory -> (&self) {}
}
