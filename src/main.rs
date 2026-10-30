//import the necessary library for interacting with hardware registers
//embedded_hal is a library tha provides a common hardware abstraction layer for embedded systems

use core::convert::Infallible; // Import the Infallible type for error handling
// use cortex_m::asm;
use embedded_hal::digital::{ErrorType, OutputPin};
use std::time::Duration; // Import the Duration type for specifying time intervals // import the ARM Cortex-M assembly macros for delaying

// Define a struct to reprent the GPIO controller
pub struct GpioController {
    pin_a: MyGpioPin, // Define a field for pin A
    pin_b: MyGpioPin, // Define a field for pin B
    pin_c: MyGpioPin, // Define a field for pin C
}

// Define a struct to represent an individual GPIO pin
pub struct MyGpioPin {
    // Ad any neccessary fields here to store the pin configuration and state
    // For simplicity, we'll just use a boolean to represent the pin state (off/on)
    is_on: bool,
}

impl GpioController {
    // Constructor to create a new instance of the GPIO controller
    pub fn new() -> Self {
        // Initialize the individual GPIO pin here (this is just or illustration)
        let pin_a = MyGpioPin { is_on: false };
        let pin_b = MyGpioPin { is_on: false };
        let pin_c = MyGpioPin { is_on: false };

        GpioController {
            pin_a,
            pin_b,
            pin_c,
        }
    }
    //Function to turn on a specific Led (GPIO pin)
    pub fn turn_on_led(&mut self, led: char) {
        match led {
            'A' => self.pin_a.set_high().unwrap(), // Turn on pin A
            'B' => self.pin_b.set_high().unwrap(), // Turn on pin B
            'C' => self.pin_c.set_high().unwrap(), // Turn on pin C
            _ => (),                               // Do nothing for invalid input
        }
    }

    // Function to turn off a specific Led (GPIO pin)
    pub fn turn_off_led(&mut self, led: char) {
        match led {
            'A' => self.pin_a.set_low().unwrap(), // Turn off pin A
            'B' => self.pin_b.set_low().unwrap(), // Turn off pin B
            'C' => self.pin_c.set_low().unwrap(), // Turn off pin C
            _ => (),                              // Do nothing for invalid input
        }
    }
}

impl ErrorType for MyGpioPin {
    type Error = Infallible;
}
// Impleement the OutputPin trait for aur custom GpioPin struct
impl OutputPin for MyGpioPin {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        // Simulate setting the pin high (on) by updating the internal state
        self.is_on = true; // Set the pin state to high (on)
        println!("Set GPIO pin high (on)"); // Print a message indicating the pin is set high
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.is_on = false; // Set the pin state to low (off)
        println!("Set GPIO pin low (off)"); // Print a message indicating the pin is set low
        Ok(())
    }
}

fn main() {
    // Create a new instance of GPIO controller
    let mut pio_controller = GpioController::new();

    // Turn on LED A
    pio_controller.turn_on_led('A');
    // Wait for some time (simulate LED being on)
    std::thread::sleep(Duration::from_secs(1)); // Delay for 1 second (this is just a placeholder, actual delay implementation may vary)
    // asm::delay(1000000); // Delay for a while (this is just a placeholder, actual delay implementation may vary)

    // Turn off LED A
    pio_controller.turn_off_led('A');
    // Wait for some time (simulate LED being off)
    std::thread::sleep(Duration::from_secs(1)); // Delay for 1 second (this is just a placeholder, actual delay implementation may vary)
}
