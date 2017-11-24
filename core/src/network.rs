use layer::Layer;

pub struct Network {
    layers: Vec<Layer>,
    activation: Fn(f32) -> f32
}

impl Network {
    pub fn compute(&self, input: &Vec<f32>) -> Vec<f32> {
        self.layers.iter()
        .fold(input.to_vec(),|layer_input, layer| layer.compute(&self.activation, &layer_input))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn testje() {
        
    }
}
