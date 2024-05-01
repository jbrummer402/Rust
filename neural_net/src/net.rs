use std::{rc::RefCell, rc::Rc}
use std::sync::{Arc, Mutex};


#[derive(Debug)]
struct Net {
    Layers : Vec<Arc<Mutex<Node>>>
}

#[derive(Debug)]
struct Node {
    weight: f32,
    gradients: Vec<f32> 
}

impl NeuralNet {
    pub fn new() -> Self {
        NeuralNet {
            Layers: Vec::new()
        }
    }

    pub fn add_layer(&mut self, num_neurons: usize, num_inputs: usize, type: LayerType) {
        let mut layer = Vec::new();
        for _ in 0..num_neurons {
            let node = Node {
                weight: 0.0,
                gradients: vec![0.0; num_inputs]
           };
            layer.push(Arc::new(Mutex::new(node)));
        }
        self.Layers.push(layer);
    }
}
