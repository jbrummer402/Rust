enum LayerType {
    Input,
    Dense,
    Activation,
    Output,
}

pub struct Layer {
    weights: Vec<f64>,
    biases: Vec<f64>,
}

impl Layer {
    pub fn new(num_neurons: usize, num_inputs: usize) -> Self {
        let mut rng = rand::thread_rng();
        let weights: Vec<Vec<f64>> = (0..num_neurons)
            .map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();
        let biases: Vec<f64> = (0..num_neurons).map(|_| rng.gen_range(-1.0..1.0)).collect();

        Layer { weights, biases }
    }

    pub fn forward(&self, inputs: &[f64]) -> Vec<f64> {
        let mut outputs: Vec<f64> = vec![0.0; self.weights.len()];

        for (neuron, (weights, bias)) in self.weights.iter().zip(&self.biases).enumerate() {
            let weighted_sum: f64 = inputs.iter().zip(weights).map(|(input, weight)| input * weight).sum();
            let output = weighted_sum + bias;
            outputs[neuron] = output;
        }

        outputs
    }
}
pub fn activate(&self, inputs: &[f64]) -> Vec<f64> {
    let outputs = self.forward(inputs);
    // Apply activation function to the outputs
    let activated_outputs: Vec<f64> = outputs.iter().map(|output| self.activation_function(*output)).collect();
    activated_outputs
}

fn activation_function(&self, x: f64) -> f64 {
    // Define your activation function here
    // Example: sigmoid function
    1.0 / (1.0 + (-x).exp())
}