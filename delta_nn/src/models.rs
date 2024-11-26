use delta_common::{Dataset, Layer, Optimizer};
use delta_common::data::DatasetOps;

#[derive(Debug)]
pub struct Sequential {
    layers: Vec<Box<dyn Layer>>,
    optimizer: Option<Box<dyn Optimizer>>,
}

impl Sequential {
    pub fn new() -> Self {
        Self { layers: Vec::new(), optimizer: None }
    }

    pub fn add<L: Layer + 'static>(mut self, layer: L) -> Self {
        self.layers.push(Box::new(layer));
        self
    }

    pub fn compile<O: Optimizer + 'static>(&mut self, optimizer: O) {
        self.optimizer = Some(Box::new(optimizer));
    }

    pub fn train(&self, train_data: &Dataset, batch_size: usize) {
        // Implement training logic here
    }

    pub fn fit<D: DatasetOps>(&self, train_data: &D, epochs: i32, batch_size: i32) {
        // Implement training logic here
    }

    pub fn validate(&self, test_data: &Dataset) -> f32 {
        // Implement validation logic here
        0.0 // Placeholder
    }

    pub fn evaluate<D: DatasetOps>(&self, test_data: &D) -> f32 {
        // Implement evaluation logic here
        0.0 // Placeholder
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        // Implement model saving logic here
        Ok(())
    }

    /*pub fn forward(&self, input: &Tensor) -> Tensor {
        self.layers.iter().fold(input.clone(), |acc, layer| layer.forward(&acc))
    }*/

    pub fn summary(&self) -> String {
        let mut summary = String::new();
        for (i, layer) in self.layers.iter().enumerate() {
            summary.push_str(&format!("Layer {}: {:?}\n", i + 1, layer));
        }
        summary
    }
}