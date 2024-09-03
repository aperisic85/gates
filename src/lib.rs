#[derive(Debug, Clone, Copy)]
pub struct Pin {
    value: u8,
    state: PinConnectionState,
}

#[derive(Debug, Clone, Copy)]
pub struct Output(Pin);

impl Output {
    pub fn get_u8_result(&self) -> u8 {
        self.0.get_value()
    }
}

#[derive(Debug, Clone, Copy)]
enum PinConnectionState {
    Connected,
    NotConnected,
}

pub trait OutputResult {
    fn calculate(self) -> Output;
}

impl Pin {
    pub const HIGH: u8 = 1;
    pub const LOW: u8 = 0;

    pub fn new_disconected() -> Self {
        Pin {
            value: Pin::HIGH,
            state: PinConnectionState::NotConnected,
        }
    }
    pub fn set_high(&mut self) {
        self.value = Pin::HIGH;
    }

    pub fn set_low(&mut self) {
        self.value = Pin::LOW;
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}

impl PartialEq<u8> for Pin {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}
pub struct AndGateElementary {
    input1: Pin,
    input2: Pin,
    output: Pin,
}

impl AndGateElementary {
    pub fn new() -> Self {
        AndGateElementary {
            input1: Pin::new_disconected(),
            input2: Pin::new_disconected(),
            output: Pin::new_disconected(),
        }
    }
}

pub struct OrGate {
    no_of_inputs: u8,
    input_pins: Vec<Pin>,
    output: Pin,
}

pub struct AndGate {
    pub no_of_inputs: u8,
    pub input_pins: Vec<Pin>,
    pub output: Pin,
}

impl AndGate {
    pub fn new(no_of_input: u8) -> Self {
        let mut pin_vec: Vec<Pin> = Vec::new();
        for _ in 0..no_of_input {
            pin_vec.push(Pin::new_disconected());
        }
        AndGate {
            no_of_inputs: no_of_input,
            input_pins: pin_vec,
            output: Pin::new_disconected(),
        }
    }
}

impl OutputResult for AndGate {
    fn calculate(self) -> Output {
        let result =
            self.input_pins.iter().fold(
                1,
                |acc, num| if (acc & num.get_value()) == 1 { 1 } else { 0 },
            );
        let mut output_pin = Pin::new_disconected();
        if result == 1 {
            output_pin.set_high();
        } else {
            output_pin.set_low();
        }
        Output(output_pin)
    }
}