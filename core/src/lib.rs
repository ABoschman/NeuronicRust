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
    use Neuron;

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

    #[test]
    fn neuron_weighted_sum_empty() {
        let neuron = Neuron {
            bias: 0.0,
            weights: vec!(),
        };
        assert_eq!(neuron.weighted_sum(&vec!()), 0.0);
    }

    #[test]
    fn neuron_weighted_sum_single_input() {
        let neuron = Neuron {
            bias: 0.0,
            weights: vec!(2.0),
        };
        assert_eq!(neuron.weighted_sum(&vec!(3.0)), 6.0);
    }

    #[test]
    fn neuron_weighted_sum_two_inputs() {
        let neuron = Neuron {
            bias: 0.0,
            weights: vec!(2.0, 3.0),
        };
        assert_eq!(neuron.weighted_sum(&vec!(5.0, 7.0)), 31.0);
    }
}


/// Converts an array of inputs to
trait Compute {
    /// Performs computation on an array of inputs to produce an array of outputs, using some computer.
    ///
    /// The number of inputs does not necessarily have to equal the number of outputs.
    /// All inputs and outputs must be real numbers in the range [0, 1].
    fn compute(&self, input: &Vec<f32>) -> Vec<f32>;
}

struct Network {
    // list of layers.
    layers: Vec<Layer>
}

struct Layer {
    neurons: Vec<Neuron>
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>
}

impl Neuron {

    fn compute(&self, input: &Vec<f32>) -> f32 {
        activation(self.bias + self.weighted_sum(input))
    }

    fn weighted_sum(&self, input: &Vec<f32>) -> f32 {
        self.weights.iter()
        .zip(input.iter())
        .map(|(w, i)| w * i)
        .sum()
    }
}

impl Compute for Layer {
    fn compute(&self, input: &Vec<f32>) -> Vec<f32> {
        self.neurons.iter()
        .map(|neuron| neuron.compute(input))
        .collect()
    }
}
impl Compute for Network {
    fn compute(&self, input: &Vec<f32>) -> Vec<f32> {
        self.layers.iter()
        .fold(input.to_vec(),|layer_input, layer| layer.compute(&layer_input))
    }
}

// impl Compute for Network {
//     fn compute(input: Vec<f32>) -> Vec<f32> {
//         //todo
//     }
// }




//
