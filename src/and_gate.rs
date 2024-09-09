use crate::gate::{Gate, Output, Pin};

pub struct AndGate {
    pub no_of_inputs: u8,
    pub input_pins: Vec<Pin>,
    pub output: Pin,
}

impl Gate for AndGate {
    fn new(no_of_input: u8) -> Result<Self, String> {
        if no_of_input <= 2 {
            Err("Number of inputs must be greater than 0".to_string())
        } else {
            let mut pin_vec: Vec<Pin> = Vec::new();
            for _ in 0..no_of_input {
                pin_vec.push(Pin::new());
            }

            Ok(AndGate {
                no_of_inputs: no_of_input,
                input_pins: pin_vec,
                output: Pin::new(),
            })
        }
    }
    fn calculate_output(self) -> Output {
        /* let result = self
        .input_pins
        .iter()
        .fold(1, |acc, num| acc & num.get_value()); */
        let result = self.input_pins.iter().all(|x| x.get_value() == 1);
        let mut output_pin = Pin::new();
        if result {
            output_pin.set_high();
        } else {
            output_pin.set_low();
        }
        Output(output_pin)
    }
}
