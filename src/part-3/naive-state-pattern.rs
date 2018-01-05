trait State {
  fn request_review(self: Box<Self>) -> Box<State>;
  fn approve(self: Box<Self>) -> Box<State>;

  fn content<'a>(&'a self, mesh: &'a Mesh) -> &'a str {
    ""
  }
}

struct PendingReview {}

struct Draft {}

struct Published {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<State> {
    Box::new(PendingReview {})
  }

  fn approve(self: Box<Self>) -> Box<State> {
    self
  }
}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<State> {
    Box::new(Published {})
  }
}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<State> {
    self
  }

  fn content<'a>(&'a self, mesh: &'a Mesh) -> &'a str {
    &mesh.content
  }
}

pub struct Mesh {
  state: Option<Box<State>>,
  content: String,
}

impl Mesh {
  pub fn new() -> Mesh {
    Mesh {
      state: Some(Box::new(Draft {})),
      content: String::new()
    }
  }

  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(&self)
  }

  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(&text);
  }

  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review())
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}

fn main() {
  let mut mesh = Mesh::new();

  mesh.add_text("Lorem Ipsum");
  assert_eq!("", mesh.content());

  mesh.request_review();
  assert_eq!("", mesh.content());

  mesh.approve();
  assert_eq!("Lorem Ipsum", mesh.content());

  println!("{}", mesh.content());
}
