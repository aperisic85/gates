use gates::{AndGate,OutputResult};

fn main() {}

#[test]
fn and_output() {
    let mut gate1 = AndGate::new(3);
    gate1.input_pins.get_mut(0).unwrap().set_low();
    gate1.input_pins.get_mut(1).unwrap().set_high();
    gate1.input_pins.get_mut(2).unwrap().set_high();

    assert_eq!(0, gate1.calculate().get_u8_result());
}

#[test]
fn and_output2() {
    let mut gate2 = AndGate::new(3);
    gate2.input_pins.get_mut(0).unwrap().set_high();
    gate2.input_pins.get_mut(1).unwrap().set_high();
    gate2.input_pins.get_mut(2).unwrap().set_high();
    let gate2_output = gate2.calculate();
    assert_eq!(1, gate2_output.get_u8_result());
}
