
#[derive(Debug,Clone, Copy)]
struct Pin(u8);

impl Pin {
    const HIGH : u8 = 1;
    const LOW : u8 = 0;

    fn high() -> Self {
        Pin(Pin::HIGH)
    }

    fn low() -> Self {
        Pin(Pin::LOW)
    }

    fn get_value(&self) -> u8 {
        self.0
    }
}

impl PartialEq<u8> for Pin {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}
struct AndGate {
    input1: Pin,
    input2: Pin,
    output: Pin,
}

impl AndGate {
    fn new(in1: Pin, in2: Pin) -> Self {
        let mut out:Pin = Pin(0);
        if in1 == Pin::HIGH && in2 == Pin::HIGH {
            out = Pin(Pin::HIGH);
        } else {
            out = Pin(Pin::HIGH);
        }
        AndGate {
            input1: in1,
            input2: in2,
            output : out,
        }
    }

   // fn chain (&self, rhs: Self) -> Self{

    //}
}

struct OrGate {
    input1: u8,
    input2: u8,
    output: u8,
}

fn main() {
    let gate1 = AndGate::new(Pin(Pin::HIGH), Pin(Pin::LOW));
    let gate2 = AndGate::new(Pin::high(), Pin::low());
    println!("Output value is : {} and {} ",gate1.output.get_value(), gate2.output.get_value());
}
