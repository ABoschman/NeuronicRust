use rand;

#[derive(Debug)]
pub struct Neuron {
    bias: f32,
    weights: Vec<f32>
}

impl Neuron {

    pub fn new_rand(nr_weights: usize) -> Neuron {
        Neuron {
            bias: rand::random::<f32>(),
            weights: (0..nr_weights).map(|i| rand::random::<f32>()).collect()
        }
    }

    pub fn new(bias: f32, weights: Vec<f32>) -> Neuron {
        Neuron {
            bias: bias,
            weights: weights
        }
    }

    pub fn compute(&self, activation: &Fn(f32) -> f32, input: &Vec<f32>) -> f32 {
        activation(self.bias + self.weighted_sum(input))
    }

    /// # Examples
    ///
    /// ```
    /// use core::neuron::Neuron;
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
    use activation::*;

    #[test]
    fn compute_activation_input_unbiased_neuron() {
        let bias = 0.0;
        let unbiased_neuron = Neuron::new(bias, vec!(0.0));
        assert_eq!(bias, unbiased_neuron.compute(&|a| a, &vec!(0.0)));
    }

    #[test]
    fn compute_activation_input_neuron_with_bias() {
        let bias = 1.0;
        let biased_neuron = Neuron::new(bias, vec!(0.0));
        assert_eq!(bias, biased_neuron.compute(&|a| a, &vec!(0.0)));
    }

}
