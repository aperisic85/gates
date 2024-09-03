#[derive(Debug, Clone, Copy)]
struct Pin {
    value: u8,
    state: PinConnectionState,
}

#[derive(Debug, Clone, Copy)]
enum PinConnectionState {
    Connected,
    NotConnected,
}

impl Pin {
    const HIGH: u8 = 1;
    const LOW: u8 = 0;

    fn new_disconected() -> Self {
        Pin {
            value: Pin::HIGH,
            state: PinConnectionState::NotConnected,
        }
    }
    fn connect_high() -> Self {
        Pin {
            value: Pin::HIGH,
            state: PinConnectionState::Connected,
        }
    }

    fn connect_low() -> Self {
        Pin {
            value: Pin::LOW,
            state: PinConnectionState::Connected,
        }
    }

    fn get_value(&self) -> u8 {
        self.value
    }
}

impl PartialEq<u8> for Pin {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}
struct AndGate {
    input1: Pin,
    input2: Pin,
    output: Pin,
}

impl AndGate {
    fn new() -> Self {
        AndGate {
            input1: Pin::new_disconected(),
            input2: Pin::new_disconected(),
            output: Pin::new_disconected(),
        }
    }

    fn connect(p: Pin, other: Pin) {}
    // fn chain (&self, rhs: Self) -> Self{

    //}
}

struct OrGate {
    input1: u8,
    input2: u8,
    output: u8,
}

fn main() {
    let gate1 = AndGate::new();

    println!(
        "Output value is : {} ",
        gate1.output.get_value(),
    );
}
