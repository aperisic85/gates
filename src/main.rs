use gates::and_gate::AndGate;
use gates::gate::*;
use gates::xor_gate::XOrGate;
fn main() {}

#[test]
fn new_and_gate_error() {
    let gate1 = AndGate::new(1);
    let _ = match gate1 {
        Ok(mut gate1) => {
            gate1.input_pins.get_mut(0).unwrap().set_low();
            assert_eq!(0, gate1.calculate_output().get_u8_result());
        }
        Err(msg) => {
            assert_eq!("Number of inputs must be greater than 0".to_string(), msg);
        }
    };
}

#[test]
fn new_and_gate_ok() {
    let gate1 = AndGate::new(3);
    let _ = match gate1 {
        Ok(mut gate1) => {
            gate1.input_pins.get_mut(0).unwrap().set_low();
            gate1.input_pins.get_mut(1).unwrap().set_high();
            gate1.input_pins.get_mut(2).unwrap().set_high();

            assert_eq!(0, gate1.calculate_output().get_u8_result());
        }
        Err(msg) => {
            assert_eq!("Number of inputs must be greater than 0".to_string(), msg);
        }
    };
}

#[test]
fn and_gate_output2() {
    let gate1 = AndGate::new(3);
    let _ = match gate1 {
        Ok(mut gate1) => {
            gate1.input_pins.get_mut(0).unwrap().set_low();
            gate1.input_pins.get_mut(1).unwrap().set_high();
            gate1.input_pins.get_mut(2).unwrap().set_high();

            assert_eq!(0, gate1.calculate_output().get_u8_result());
        }
        Err(msg) => {
            assert_eq!("Number of inputs must be greater than 0".to_string(), msg);
        }
    };
}
