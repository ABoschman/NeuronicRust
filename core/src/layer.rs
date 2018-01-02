use neuron::Neuron;

#[derive(Debug)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {

    pub fn new_rand(nr_inputs: usize, nr_neurons: usize) -> Layer {
        Layer {
            neurons: (0..nr_neurons).map(|i| Neuron::new_rand(nr_inputs)).collect()
        }
    }

    pub fn compute(&self, activation: &Fn(f32) -> f32, input: &Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.compute(&activation, input))
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

}
