//! BSD 3-Clause License
//!
//! Copyright (c) 2024, Marcus Cvjeticanin, Chase Willden
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met:
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from
//!    this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::common::data::{Dataset, DatasetOps};
use crate::common::layer::Layer;
use crate::common::loss::Loss;
use crate::common::optimizer::Optimizer;

/// A sequential model that contains a list of layers, an optimizer, and a loss function.
#[derive(Debug)]
pub struct Sequential {
    layers: Vec<Box<dyn Layer>>,
    layer_names: Vec<String>,
    optimizer: Option<Box<dyn Optimizer>>,
    loss: Option<Box<dyn Loss>>,
}

impl Sequential {
    /// Creates a new sequential model.
    ///
    /// # Returns
    ///
    /// A new instance of the sequential model.
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            layer_names: Vec::new(),
            optimizer: None,
            loss: None,
        }
    }

    /// Add a layer to the model
    ///
    /// # Arguments
    ///
    /// * `layer` - The layer to add
    ///
    /// # Returns
    ///
    /// A reference to the model
    pub fn add<L: Layer + 'static>(mut self, mut layer: L) -> Self {
        let layer_type = std::any::type_name::<L>().split("::").last().unwrap();
        let layer_name = match layer_type {
            "Dense" => format!("{}_{}_{}", layer_type, layer.units(), self.layers.len()),
            _ => format!("{}_{}", layer_type, self.layers.len()),
        };

        // Call the build method to initialize weights and biases
        if !self.layers.is_empty() {
            let input_shape = self.layers.last().unwrap().output_shape();
            layer.build(input_shape);
        }

        self.layers.push(Box::new(layer));
        self.layer_names.push(layer_name);
        self
    }

    /// Compiles the model with the given optimizer and loss function.
    ///
    /// # Arguments
    ///
    /// * `optimizer` - The optimizer to use.
    /// * `loss` - The loss function to use.
    pub fn compile<O: Optimizer + 'static, L: Loss + 'static>(&mut self, optimizer: O, loss: L) {
        self.optimizer = Some(Box::new(optimizer));
        self.loss = Some(Box::new(loss));
    }

    /// Trains the model with the given training data and batch size.
    ///
    /// # Arguments
    ///
    /// * `train_data` - The training data.
    /// * `batch_size` - The batch size to use.
    pub fn train(&self, train_data: &Dataset, batch_size: usize) {
        let _ = batch_size;
        let _ = train_data;
        // Implement training logic here
    }

    /// Trains the model with the given training data, number of epochs, and batch size.
    ///
    /// # Arguments
    ///
    /// * `train_data` - The training data.
    /// * `epochs` - The number of epochs to train for.
    /// * `batch_size` - The batch size to use.
    pub fn fit<D: DatasetOps>(&mut self, train_data: &mut D, epochs: i32, batch_size: usize) {
        // Ensure optimizer is set
        if self.optimizer.is_none() {
            panic!("Optimizer must be set before training");
        }

        let mut optimizer = self.optimizer.as_mut().unwrap();

        for epoch in 0..epochs {
            println!("Epoch {}/{}", epoch + 1, epochs);

            // Shuffle dataset if necessary
            let dataset = train_data.clone();

            // TODO: Probably should implement a shuffle capability
            // dataset.shuffle();

            let num_batches = dataset.len() / batch_size;
            let mut epoch_loss = 0.0;

            for batch_idx in 0..num_batches {
                // Fetch batch
                let (inputs, targets) = dataset.get_batch(batch_idx, batch_size);

                // Forward pass
                let mut outputs = inputs.clone();
                for layer in &mut self.layers {
                    outputs = layer.forward(&outputs);
                }

                // Compute loss
                if self.loss.is_some() {
                    let loss = self.loss.as_ref().unwrap();
                    epoch_loss += loss.calculate_loss(&outputs, &targets);
                } else {
                    epoch_loss += 0.0;
                }

                // Backward pass
                let mut grad = train_data.loss_grad(&outputs, &targets);
                for layer in self.layers.iter_mut().rev() {
                    grad = layer.backward(&grad);
                }

                // Update weights
                for layer in &mut self.layers {
                    layer.update_weights(&grad, &mut optimizer);
                }
            }

            println!(
                "Epoch {} completed. Average Loss: {:.4}",
                epoch + 1,
                epoch_loss / num_batches as f32
            );
        }
    }

    /// Validates the model with the given test data.
    ///
    /// # Arguments
    ///
    /// * `test_data` - The test data.
    ///
    /// # Returns
    ///
    /// The validation loss.
    pub fn validate(&self, test_data: &Dataset) -> f32 {
        let _ = test_data;
        // Implement validation logic here
        0.0 // Placeholder
    }

    /// Evaluates the model with the given test data.
    ///
    /// # Arguments
    ///
    /// * `test_data` - The test data.
    ///
    /// # Returns
    ///
    /// The evaluation metric.
    pub fn evaluate<D: DatasetOps>(&self, test_data: &D) -> f32 {
        let _ = test_data;
        // Implement evaluation logic here
        0.0 // Placeholder
    }

    /// Saves the model to the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to save the model to.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let _ = path;
        // Implement model saving logic here
        Ok(())
    }

    /*pub fn forward(&self, input: &Tensor) -> Tensor {
        self.layers.iter().fold(input.clone(), |acc, layer| layer.forward(&acc))
    }*/

    /// Prints a summary of the model.
    pub fn summary(&self) {
        println!("Model Summary:");
        println!(
            "{:<30} {:<25} {:<10}",
            "Layer (type)", "Output Shape", "Param #"
        );
        println!("{:-<65}", "");

        let mut total_params = 0;
        let mut trainable_params = 0;
        let mut non_trainable_params = 0;

        for (i, layer) in self.layers.iter().enumerate() {
            let layer_type = &self.layer_names[i];
            let layer_type_only = layer_type.split('_').next().unwrap();
            let display_name = format!("{} ({})", layer_type, layer_type_only);
            let output_shape = format!("{:?}", layer.output_shape());
            let (trainable, non_trainable) = layer.param_count();
            total_params += trainable + non_trainable;
            trainable_params += trainable;
            non_trainable_params += non_trainable;
            println!(
                "{:<30} {:<25} {:<10}",
                display_name,
                output_shape,
                trainable + non_trainable
            );
        }

        println!("{:-<65}", "");
        println!("Total params: {}", total_params);
        println!("Trainable params: {}", trainable_params);
        println!("Non-trainable params: {}", non_trainable_params);
        println!("{:-<65}", "");
    }
}
