pub trait Gate {
    fn new(no_inputs: u8) -> Result<Self, String>
    where
        Self: Sized;
    fn calculate_output(self) -> Output;
}

#[derive(Debug, Clone, Copy)]
pub struct Pin {
    pub value: u8,
    state: PinConnectionState,
}
impl Pin {
    pub const HIGH: u8 = 1;
    pub const LOW: u8 = 0;

    pub fn new() -> Self {
        Pin {
            value: Pin::LOW,
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
#[derive(Debug, Clone, Copy)]
pub struct Output(pub Pin);

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

pub struct AndGateElementary {
    input1: Pin,
    input2: Pin,
    output: Pin,
}

impl AndGateElementary {
    pub fn new() -> Self {
        AndGateElementary {
            input1: Pin::new(),
            input2: Pin::new(),
            output: Pin::new(),
        }
    }
}
