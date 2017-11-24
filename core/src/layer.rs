use neuron::Neuron;

pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn compute(&self, activation: &Fn(f32) -> f32, input: &Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.compute(&activation, input))
            .collect()
    }
}
