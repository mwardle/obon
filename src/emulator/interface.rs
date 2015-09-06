pub trait Interface {
  // information
  fn title(&self) -> String;
  fn videoFrequency(&self) -> f64;
  fn audioFrequency(&self) -> f64;

  // media
  fn loaded(&self) -> bool { false }
  fn sha256(&self) -> String { "".to_string() }
  fn group(&self, id: usize) -> usize;
  fn load(&mut self) -> () {}
  fn save(&mut self) -> () {}
  fn unload(&mut self) -> () {}

  // system
  fn connect(&mut self, port: usize, device: usize) -> () {}
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
  fn cheatSet(&self, lstring: Option<&'static str> ) -> () {}

  // utility functions
  // TODO

  // debugger functions
  fn tracerEnable(&self, bool) -> bool { false }
  fn exportMemory(&self) -> () {}

  // return common data
  fn information(&self) -> Information;
  fn media(&self) -> Vec<Media>;
  fn port(&self) -> Vec<Port>;

  // TODO Bind forwarding calls??
  //      how should this be done??
  //      what does it even do??
}

// I don't know what this does yet
trait Bind {
//  fn loadRequest(&mut self, usize, String, Option<String>) -> () {}
//  fn saveRequest(&mut self, usize, String) -> () {}
//  fn videoColor(&self, usize, u16, u16, u16, u16) -> u32 { 0 }
//  fn videoRefresh(&mut self, u32, u32, usize, usize, usize) -> () {}
//  fn audioSample(&self, i16, i16) -> () {}
//  fn inputPoll(&self, usize, usize, usize) -> i16 { 0 }
//  // TODO?? fn dipSettings(&self, Markup::Node) -> usize
//  fn path(&self) -> String { "".to_string() }
//  fn server(&self) -> String { "".to_string() }
//  fn notify(&self, text: String) -> () { println!("{}", text); }
}

pub struct Capability {
  states: bool,
  cheats: bool,
}

pub struct Information {
  name: String,
  width: usize,
  height: usize,
  overscan: bool,
  resettable: bool,
  capability: Capability,
}

pub struct Media {
  id: usize,
  name: String,
  mediaType: String,
  bootable: bool,
}

pub enum InputType {
  DIGITAL,
  ANALOG,
  RUMBLE,
}

pub struct Input {
  id: usize,
  inputType: InputType,
  name: String,
  guid: usize,
}

pub struct Device {
  id: usize,
  portmask: usize,
  name: String,
  input: Vec<Input>,
  order: Vec<usize>,
}

pub struct Port {
  id: usize,
  name: String,
  device: Vec<Device>,
}
