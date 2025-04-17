use std::{cmp::Ordering, path::Path};

use tensorflow::{Graph, SavedModelBundle, SessionOptions, SessionRunArgs, Status, Tensor};

use crate::Board;

use super::board_to_network_input;

pub struct HBSessionHandle {
    graph: Graph,
    bundle: SavedModelBundle,
}

impl HBSessionHandle {
    pub fn load(path: Option<&Path>) -> Self {
        let path: &Path = path.unwrap_or(&Path::new(r"model"));

        let mut graph = Graph::new();
        let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, path)
            .expect("Can't load saved model");

        Self { graph, bundle }
    }

    pub fn call(&self, lhs: &Tensor<f32>, rhs: &Tensor<f32>) -> Result<Tensor<f32>, Status> {
        let call_signature = self
            .bundle
            .meta_graph_def()
            .get_signature("call")
            .expect("Signature 'call' not found in saved_mode.pb");

        let lhs_input_info = call_signature.get_input("inputs")?;
        let lhs_input_op = self
            .graph
            .operation_by_name(&lhs_input_info.name().name)?
            .unwrap();
        let rhs_input_info = call_signature.get_input("inputs_1")?;
        let rhs_input_op = self
            .graph
            .operation_by_name(&rhs_input_info.name().name)?
            .unwrap();

        let output_info = call_signature.get_output("output_0")?;
        let output_op = self
            .graph
            .operation_by_name(&output_info.name().name)?
            .unwrap();

        let mut call_step = SessionRunArgs::new();
        call_step.add_feed(&lhs_input_op, 0, lhs);
        call_step.add_feed(&rhs_input_op, 0, rhs);
        let output = call_step.request_fetch(&output_op, 0);
        self.bundle.session.run(&mut call_step)?;

        Ok(call_step.fetch(output)?)
    }

    pub fn call_encoded(
        &self,
        lhs: &Tensor<f32>,
        rhs: &Tensor<f32>,
    ) -> Result<Tensor<f32>, Status> {
        let call_signature = self
            .bundle
            .meta_graph_def()
            .get_signature("call_encoded")
            .expect("Signature 'call_encoded' not found in saved_mode.pb");

        let lhs_input_info = call_signature.get_input("inputs")?;
        let lhs_input_op = self
            .graph
            .operation_by_name(&lhs_input_info.name().name)?
            .unwrap();
        let rhs_input_info = call_signature.get_input("inputs_1")?;
        let rhs_input_op = self
            .graph
            .operation_by_name(&rhs_input_info.name().name)?
            .unwrap();

        let output_info = call_signature.get_output("output_0")?;
        let output_op = self
            .graph
            .operation_by_name(&output_info.name().name)?
            .unwrap();

        let mut call_step = SessionRunArgs::new();
        call_step.add_feed(&lhs_input_op, 0, lhs);
        call_step.add_feed(&rhs_input_op, 0, rhs);
        let output = call_step.request_fetch(&output_op, 0);
        self.bundle.session.run(&mut call_step)?;

        Ok(call_step.fetch(output)?)
    }

    pub fn encode(&self, input: Tensor<f32>) -> Result<Tensor<f32>, Status> {
        let call_signature = self
            .bundle
            .meta_graph_def()
            .get_signature("encode")
            .expect("Signature 'encode' not found in saved_mode.pb");

        let input_info = call_signature.get_input("input")?;
        let input_op = self
            .graph
            .operation_by_name(&input_info.name().name)?
            .unwrap();

        let output_info = call_signature.get_output("output_0")?;
        let output_op = self
            .graph
            .operation_by_name(&output_info.name().name)?
            .unwrap();

        let mut call_step = SessionRunArgs::new();
        call_step.add_feed(&input_op, 0, &input);
        let output = call_step.request_fetch(&output_op, 0);
        self.bundle.session.run(&mut call_step)?;

        Ok(call_step.fetch(output)?)
    }

    /// Compares two [`Board`]s. Returns [`Ordering::Greater`]
    /// if `lhs` is a better position than `rhs`, [`Ordering::Equal`]
    /// if they are equally good and [`Ordering::Less`] otherwise
    pub fn compare(&self, lhs: &Board, rhs: &Board) -> Result<Ordering, Status> {
        let lhs_input = board_to_network_input(lhs, lhs.to_play());
        let rhs_input = board_to_network_input(rhs, rhs.to_play());
        let result = self.call(&lhs_input.into(), &rhs_input.into())?;

        Ok(if result[0] > result[1] {
            Ordering::Greater
        } else if result[0] < result[1] {
            Ordering::Less
        } else {
            Ordering::Equal
        })
    }

    /// Compares two encoded [`Board`]s. Returns [`Ordering::Greater`]
    /// if `lhs` is a better position than `rhs`, [`Ordering::Equal`]
    /// if they are equally good and [`Ordering::Less`] otherwise
    pub fn compare_encoded(
        &self,
        lhs: &Tensor<f32>,
        rhs: &Tensor<f32>,
    ) -> Result<(Ordering, Tensor<f32>), Status> {
        let result = self.call_encoded(lhs, rhs)?;

        Ok((
            if result[0] > result[1] {
                Ordering::Greater
            } else if result[0] < result[1] {
                Ordering::Less
            } else {
                Ordering::Equal
            },
            result,
        ))
    }
}
