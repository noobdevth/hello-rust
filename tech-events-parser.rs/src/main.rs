extern crate pulldown_cmark;

use std::fs::File;
use std::io::prelude::*;

use pulldown_cmark::Parser;
use pulldown_cmark::Tag::*;
use pulldown_cmark::Event::*;

// Flags
#[derive(Debug, Clone)]
pub enum Detail {
  Topic(String),
  Place(String),
  Time(String),
  Website(String),
  Ticket(String),
  RSVP(String)
}

#[derive(Debug, Clone)]
pub struct Event {
  name: String,
  description: String,
  note: String,
  date: String,
  details: Vec<Detail>
}

impl Event {
  fn new() -> Event {
    Event {
      name: format!(""),
      description: format!(""),
      note: format!(""),
      date: format!(""),
      details: vec![]
    }
  }
}

#[derive(Debug)]
pub struct EventParser {
  input: String,
  event: Event,
  events: Vec<Event>,
  in_list: bool,
  in_event: bool,
  in_title: bool,
  in_desc: bool,
  in_note: bool,
  halt: bool
}

impl EventParser {
  fn new(input: &str) -> EventParser {
    EventParser {
      input: String::from(input),
      event: Event::new(),
      events: vec![],
      in_list: false,
      in_event: false,
      in_title: false,
      in_desc: false,
      in_note: false,
      halt: false
    }
  }

  fn start_h2(&mut self) {
    println!("> Encountered Header(2)");

    if !self.in_list {
      self.in_list = true;

      println!("--- Beginning of List ---");
    } else {
      self.in_list = false;
      self.halt = true;

      println!("--- End of List --- ");
    }
  }

  fn start_h3(&mut self) {
    println!("> Encountered Header(3) | In List: {} | In Topic: {}", self.in_list, self.in_event);

    if self.in_list {
      // First, end the current event.
      // This won't get called on the first event, because there isn't any event to end yet.
      if self.in_event {
        println!("Event: {:?}", self.event);
        self.events.push(self.event.clone());

        self.in_event = false;
        println!("--- End of Event: {} ---", self.event.name);
      }

      // Then, handle the event creation
      if !self.in_event {
        println!("--- Beginning of Event ---");

        self.event = Event::new();
      }

      // When H3 is encountered, mark that we're in an event and the title tag.
      self.in_event = true;
      self.in_title = true;
    }
  }

  fn parse(&mut self) {
    let input = self.input.clone();
    let parser = Parser::new(&input);

    for tag in parser {
      if !self.halt {
        match tag {
          Start(item) => {
            match item {
              Header(h) if h == 2 => self.start_h2(),
              Header(h) if h == 3 => self.start_h3(),
              Paragraph if !self.in_note => {
                if self.in_event {
                  self.in_desc = true;
                }
              },
              BlockQuote => {
                if self.in_event {
                  self.in_note = true;
                }
              },
              _ => println!("IGNORED TAG {:?}", item)
            };
          },
          End(item) => {
            if self.in_event {
              match item {
                Header(h) if h == 3 => {
                  self.in_title = false;
                },
                Paragraph => {
                  self.in_desc = false;
                },
                BlockQuote => {
                  self.in_note = false;
                },
                _ => {}
              }
            }
          },
          Text(text) => {
            if self.in_event {
              let text = String::from(text);
              let e = &mut self.event;

              println!("Text -> {}", text);

              if self.in_title {
                println!("TOPIC: {}", text);
                e.name = text;
              } else if self.in_desc {
                println!("DESCRIPTION: {}", text);
                e.description.push_str(&text);
              } else if self.in_note {
                println!("NOTE: {}", text);
                e.note.push_str(&text);
              }
            }
          }
          _ => continue,
        }
      }
    }
  }
}

fn main() {
  let filename = "input.md";

  let mut file = File::open(filename).expect("File Not Found");
  let mut contents = String::new();

  file
    .read_to_string(&mut contents)
    .expect("Cannot Read File");

  let mut parser = EventParser::new(&contents);
  parser.parse();

  parser.input = String::from("");
  println!("Parser State: {:?}", parser);
  println!("-----------------------------------");

  for event in parser.events {
    println!("Name: {}", event.name);
    println!("Desc: {}", event.description);
    println!("Note: {}", event.note);
    println!("-----------------------------------");
  }
}
