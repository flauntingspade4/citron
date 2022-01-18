use crate::PlayableTeam;

use tch::{nn::Linear, IndexOp};

pub struct NnueAccumulator {
    white: [f64; 40960],
    black: [f64; 40960],
}

impl NnueAccumulator {
    pub fn white_mut(&mut self) -> &mut [f64] {
        &mut self.white
    }
    pub fn black_mut(&mut self) -> &mut [f64] {
        &mut self.black
    }

    pub fn refresh(&mut self, layer: Linear, active_features: Vec<i64>, to_play: PlayableTeam) {
        if let Some(bias) = layer.bs {
            match to_play {
                PlayableTeam::White => {
                    for (white, bias) in
                        self.white_mut().iter_mut().zip(bias.iter::<f64>().unwrap())
                    {
                        *white = bias
                    }

                    for feature in active_features {
                        for (index, white) in self.white_mut().iter_mut().enumerate() {
                            *white += layer.ws.i(feature).double_value(&[index as i64]);
                        }
                    }
                }
                PlayableTeam::Black => {
                    for (black, bias) in
                        self.black_mut().iter_mut().zip(bias.iter::<f64>().unwrap())
                    {
                        *black = bias
                    }

                    for feature in active_features {
                        for (index, black) in self.black_mut().iter_mut().enumerate() {
                            *black += layer.ws.i(feature).double_value(&[index as i64]);
                        }
                    }
                }
            }
        }
    }
    pub fn update(
        &mut self,
        layer: Linear,
        mut previous_accumulator: Self,
        added_features: Vec<i64>,
        removed_features: Vec<i64>,
        to_play: PlayableTeam,
    ) {
        match to_play {
            PlayableTeam::White => {
                for (new_white, old_white) in self
                    .white_mut()
                    .iter_mut()
                    .zip(previous_accumulator.white_mut().iter())
                {
                    *new_white = *old_white
                }

                for feature in removed_features {
                    for (index, white) in self.white_mut().iter_mut().enumerate() {
                        *white -= layer.ws.i(feature).double_value(&[index as i64]);
                    }
                }

                for feature in added_features {
                    for (index, white) in self.white_mut().iter_mut().enumerate() {
                        *white += layer.ws.i(feature).double_value(&[index as i64]);
                    }
                }
            }
            PlayableTeam::Black => {
                for (new_black, old_black) in self
                    .black_mut()
                    .iter_mut()
                    .zip(previous_accumulator.black_mut().iter())
                {
                    *new_black = *old_black
                }

                for feature in removed_features {
                    for (index, black) in self.black_mut().iter_mut().enumerate() {
                        *black -= layer.ws.i(feature).double_value(&[index as i64]);
                    }
                }

                for feature in added_features {
                    for (index, black) in self.black_mut().iter_mut().enumerate() {
                        *black += layer.ws.i(feature).double_value(&[index as i64]);
                    }
                }
            }
        }
    }
}

pub fn linear(layer: Linear, output: &mut [f64], input: &[f64]) -> usize {
    if let Some(layer_bias) = layer.bs {
        for (output, bias) in output.iter_mut().zip(layer_bias.iter::<f64>().unwrap()) {
            *output = bias;
        }
    }

    for i in 0..layer.ws.numel() {
        for j in 0..layer.ws.numel() {
            output[j] += input[i] * layer.ws.i(i as i64).double_value(&[j as i64]);
        }
    }

    layer.ws.numel()
}

pub fn crelu(output: &mut [f64], input: &[f64]) {
    for (output, input) in output.iter_mut().zip(input.iter()) {
        let max = if *input > 0. { *input } else { 0. };
        *output = if max < 1. { max } else { 1. };
    }
}
