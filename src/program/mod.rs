use ::emulator;

pub struct Program {
  // TODO: add emulators
  //       a vector is troublesome...unfortunately
  // TODO: need ananke

  pause: bool,
  autopause: bool,
  depth: usize,  // color depth; 24(bpp) or 30(bpp)

  basepath: String,
  userpath: String,
  sharedpath: String,

  normalFont: String,
  boldFont: String,
  titleFont: String,
  monospaceFont: String,
}

// methods for Program
impl Program {

  fn focused(&self) -> bool {
    // TODO
    unimplemented!()
  }

  fn path(&self, filename: String) -> String {
    // TODO
    unimplemented!()
  }

  fn main(&self) -> () {
    // TODO
    unimplemented!()
  }

  fn bootstrap(&self) -> () {
    // TODO
    unimplemented!()
  }

  // static constructor method
  fn new() -> Program {
    // TODO
    unimplemented!()
  }
}
