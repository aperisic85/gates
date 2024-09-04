use gates::{AndGate, Gate, XOrGate};

fn main() {}

#[test]
fn and_gate_output() {
    let mut gate1 = AndGate::new(3);
    gate1.input_pins.get_mut(0).unwrap().set_low();
    gate1.input_pins.get_mut(1).unwrap().set_high();
    gate1.input_pins.get_mut(2).unwrap().set_high();

    assert_eq!(0, gate1.calculate_output().get_u8_result());
}

#[test]
fn and_gate_output2() {
    let mut gate2 = AndGate::new(3);
    gate2.input_pins.get_mut(0).unwrap().set_high();
    gate2.input_pins.get_mut(1).unwrap().set_high();
    gate2.input_pins.get_mut(2).unwrap().set_high();
    let gate2_output = gate2.calculate_output();
    assert_eq!(1, gate2_output.get_u8_result());
}

#[test]
fn xor_gate_output() {
    let mut xorgate = XOrGate::new(3);
    xorgate.input_pins.get_mut(0).unwrap().set_high();
    xorgate.input_pins.get_mut(1).unwrap().set_low();
    xorgate.input_pins.get_mut(2).unwrap().set_high();
    assert_eq!(0, xorgate.calculate_output().get_u8_result());
}
#[test]
fn xor_gate_output2() {
    let mut xorgate = XOrGate::new(3);
    xorgate.input_pins.get_mut(0).unwrap().set_low();
    xorgate.input_pins.get_mut(1).unwrap().set_high();
    xorgate.input_pins.get_mut(2).unwrap().set_low();
    assert_eq!(1, xorgate.calculate_output().get_u8_result());
}
