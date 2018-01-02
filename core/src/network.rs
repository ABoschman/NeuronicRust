use layer::Layer;
use std::iter;
use std::fmt;

// #[derive(Debug)]
pub struct Network<'a> {
    layers: Vec<Layer>,
    activation: &'a Fn(f32) -> f32
}

impl <'a> fmt::Display for Network<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.layers)
    }
}

impl<'a> Network<'a> {

    pub fn new_rand(activation: &'a Fn(f32) -> f32, nr_inputs: usize, neurons_per_layer: Vec<usize>) -> Network {
        let inputs = iter::once(&nr_inputs).chain(neurons_per_layer.iter().take(neurons_per_layer.len() - 1)).cloned();
        let lengths = neurons_per_layer.iter().cloned();
        Network {
            layers: inputs.zip(lengths)
                .map(|(nr_inputs, nr_neurons)| Layer::new_rand(nr_inputs, nr_neurons))
                .collect(),
            activation: activation
        }
    }

    pub fn compute(&self, input: &Vec<f32>) -> Vec<f32> {
        self.layers.iter()
        .fold(input.to_vec(), |layer_input, layer| layer.compute(&self.activation, &layer_input))
    }
}

fn act(a: f32) -> f32 {
    a
}

#[cfg(test)]
mod tests {

    use super::*;
    use activation::*;

    #[test]
    fn testje() {
        println!("{}", Network::new_rand(&sigmoid, 2, vec!(2, 2)));
    }
}
