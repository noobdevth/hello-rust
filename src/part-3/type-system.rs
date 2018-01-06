pub struct Mesh {
  content: String
}

pub struct Draft {
  content: String
}

impl Mesh {
  pub fn new() -> Draft {
    Draft { content: String::new() }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

impl Draft {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReview {
    PendingReview { content: self.content }
  }
}

pub struct PendingReview {
  content: String
}

impl PendingReview {
  pub fn approve(self) -> Mesh {
    Mesh { content: self.content }
  }
}

fn main() {
  let mut mesh = Mesh::new();

  mesh.add_text("Lorem Ipsum");

  let mesh = mesh.request_review();

  let mesh = mesh.approve();
  assert_eq!("Lorem Ipsum", mesh.content());

  println!("{}", mesh.content());
}
