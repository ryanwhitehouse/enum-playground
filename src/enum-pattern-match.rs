enum Vehicle {
    Car(String),
    MotorCycle(String),
    Bicycle(String)
}

fn main() {
    let vehicle = Vehicle::Bicycle("blue".to_string());
  
    match vehicle {
      Vehicle::Car(color) => println!("I am {} and have four tires", color),
      Vehicle::MotorCycle(color) => println!("I am {} and have two tires and run on gas", color),
      Vehicle::Bicycle(color) => println!("I am {} and have two tires and run on your effort", color)
    }
  }
