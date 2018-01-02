pub trait Draw {
  fn draw(&self);
}

#[derive(Debug)]
pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String
}

impl Draw for Button {
  fn draw(&self) {
    println!("Drawing Button | {:?}", self);
  }
}

pub struct Screen {
  pub items: Vec<Box<Draw>>
}

impl Screen {
  pub fn run(&self) {
    for item in self.items.iter() {
      item.draw();
    }
  }
}

fn main() {
  let screen = Screen {
    items: vec![
      Box::new(Button {
        width: 5,
        height: 5,
        label: String::from("Yo!")
      })
    ]
  };

  screen.run();
}

