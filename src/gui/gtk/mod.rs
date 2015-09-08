trait Program {

}

struct GUI<P: Program> {
  program: P,
}

impl GUI<P> {
  fn new (_program: P) {
    GUI {program: _program}
  }

  fn main(&mut self) {

  }
}
