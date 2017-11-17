#![allow(dead_code, unused_variables)]

/// Neural network to model an OR gate.
/// Two input nodes, input either 0 or 1.
/// One output node.
/// All weights set to 1.
/// All biasses set to 0.

/// Input = []  Layer1 = []length=nrNodesInLayer1[bias, weights...]length=nrInputs+1
/// To get activations for layer1,

fn compute(layer: [[f32; 3]; 1], input: [f32; 2]) -> f32 {
    activation(layer[0][0] + layer[0][1] * input[0] + layer[0][2] * input[1])
}

///Activation function, takes a real number and turns it into a value between 0 and 1.
fn activation(value: f32) -> f32 {
    if value > 0.9 { 1.0 } else { 0.0 }
}

#[cfg(test)]
mod tests {

    use compute;

    fn or_gate() -> [[f32; 3]; 1] {
        [[0.0, 1.0, 1.0]]
    }

    #[test]
    fn mimic_or_gate_both_off() {
        let input: [f32; 2] = [0.0, 0.0];
        assert!(compute(or_gate(), input) < 0.1);
    }

    #[test]
    fn mimic_or_gate_both_on() {
        let input: [f32; 2] = [1.0, 1.0];
        assert!(compute(or_gate(), input) > 0.9);
    }

    #[test]
    fn mimic_or_gate_one_on() {
        let input: [f32; 2] = [1.0, 0.0];
        assert!(compute(or_gate(), input) > 0.9);
    }

    #[test]
    fn mimic_or_gate_other_on() {
        let input: [f32; 2] = [0.0, 1.0];
        assert!(compute(or_gate(), input) > 0.9);
    }
}
