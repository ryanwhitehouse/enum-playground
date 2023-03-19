use std::fmt;
// enums3.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

enum Message {
  // TODO: implement the message variant types based on their usage below
  Quit,
  Move(Point),
  Echo(String),
  ChangeColor(Color)
}

#[derive(Debug)]
struct Point {
  x: u8,
  y: u8,
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      writeln!(f, "Position: ({}, {})", self.x, self.y)
  }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Color(u8, u8, u8);

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "Color: ({}, {}, {})", self.0, self.1, self.2)
  }
}

#[derive(Debug)]
struct State {
  color: Color,
  position: Point,
  quit: bool,
}

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      writeln!(f, "{}{}{}", self.color, self.position, self.quit)
  }
}

impl State {
  fn change_color(&mut self, color: Color) {
      self.color = color;
  }

  fn quit(&mut self) {
      self.quit = true;
  }

  fn echo(&self, s: String) {
      println!("{}", s);
  }

  fn move_position(&mut self, p: Point) {
      self.position = p;
  }

  fn process(&mut self, message: Message) {
      match message {
          Message::ChangeColor(color) => { 
              self.change_color(color); 
          },
          Message::Quit => { 
              self.quit(); 
          },
          Message::Echo(text) => { 
              self.echo(text);
          },
          Message::Move(point) => {
              self.move_position(point);
          }
      }
  }
}


fn main() {
    let mut state = State {
      quit: false,
      position: Point { x: 0, y: 0 },
      color: Color(0, 0, 0)
    };

    state.process(Message::ChangeColor(Color(255, 0, 255)));
    state.process(Message::Echo(String::from("hello world")));
    state.process(Message::Move(Point { x: 10, y: 15 }));
    state.process(Message::Quit);

    assert_eq!(state.color, Color(255, 0, 255));
    assert_eq!(state.position.x, 10);
    assert_eq!(state.position.y, 15);
    assert_eq!(state.quit, true);

    println!("Successfully ran.  State is {}", state);
    println!("{:#?}", state);
}

