#![allow(dead_code, unused_variables)]

struct Network {
    layers: Vec<Layer>,
    activation: Fn(f32) -> f32
}

struct Layer {
    neurons: Vec<Neuron>
}

pub struct Neuron {
    bias: f32,
    weights: Vec<f32>
}

impl Network {
    fn compute(&self, input: &Vec<f32>) -> Vec<f32> {
        self.layers.iter()
        .fold(input.to_vec(),|layer_input, layer| layer.compute(&self.activation, &layer_input))
    }
}

impl Layer {
    fn compute(&self, activation: &Fn(f32) -> f32, input: &Vec<f32>) -> Vec<f32> {
        self.neurons.iter()
        .map(|neuron| neuron.compute(&activation, input))
        .collect()
    }
}

impl Neuron {

    pub fn new(bias: f32, weights: Vec<f32>) -> Neuron {
        Neuron {
            bias: bias,
            weights: weights
        }
    }

    fn compute(&self, activation: &Fn(f32) -> f32, input: &Vec<f32>) -> f32 {
        activation(self.bias + self.weighted_sum(input))
    }

    /// # Examples
    ///
    /// ```
    /// use core::Neuron;
    ///
    /// let neuron_no_inputs = Neuron::new(0.0, vec!());
    /// assert_eq!(neuron_no_inputs.weighted_sum(&vec!()), 0.0);
    ///
    /// let neuron_one_input = Neuron::new(0.0, vec!(2.0));
    /// assert_eq!(neuron_one_input.weighted_sum(&vec!(3.0)), 6.0);
    ///
    /// let neuron_two_inputs = Neuron::new(0.0, vec!(2.0, 3.0));
    /// assert_eq!(neuron_two_inputs.weighted_sum(&vec!(5.0, 7.0)), 31.0);
    /// ```
    pub fn weighted_sum(&self, input: &Vec<f32>) -> f32 {
        self.weights.iter()
        .zip(input.iter())
        .map(|(w, i)| w * i)
        .sum()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

}
