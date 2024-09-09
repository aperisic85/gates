use crate::gate::{Gate, Output, Pin};
pub struct XOrGate {
    pub no_of_inputs: u8,
    pub input_pins: Vec<Pin>,
    pub output: Pin,
}

impl Gate for XOrGate {
    fn new(no_of_input: u8) -> Result<Self, String> {
        if no_of_input < 2 {
            Err("Number of inputs must be greater than 1".to_string())
        }
        else {
            let mut pin_vec: Vec<Pin> = Vec::new();
            for _ in 0..no_of_input {
                pin_vec.push(Pin::new());
            }
        
            Ok(XOrGate {
                no_of_inputs: no_of_input,
                input_pins: pin_vec,
                output: Pin::new(),
            })
        }
    }

    fn calculate_output(self) -> Output {
        let init_val = self.input_pins.get(0).unwrap().get_value();
        let result = self.input_pins.iter().skip(1).fold(
            //skip first element because its value is init value of acc
            init_val,
            |acc, num| acc ^ num.get_value(),
        );
        let mut output_pin = Pin::new();
        if result == 1 {
            output_pin.set_high();
        } else {
            output_pin.set_low();
        }
        Output(output_pin)
    }
}
