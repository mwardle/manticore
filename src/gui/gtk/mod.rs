trait Program {

}

struct GUI<P: Program> {
  program: P,
}

impl GUI {
  fn new<P: Program> (_program: P) {
    GUI {program: program}
  }

  fn main(&mut self) {

  }
}
