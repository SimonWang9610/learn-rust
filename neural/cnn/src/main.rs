extern crate autograd as ag;
extern crate data;
extern crate ndarray;
extern crate rand;

use ag::ndarray_ext as array;
use ag::optimizers::adam;
use ag::rand::seq::SliceRandom;
use ag::tensor::Variable;
use ag::Graph;
use ndarray::s;
// use rand::prelude::*;
// use rand::thread_rng;
use std::time::Instant;

type Tensor<'graph> = ag::Tensor<'graph, f32>;
use data::mnist;

macro_rules! timeit {
    ($x:expr) => {{
        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        println!(
            "{}.{:03} sec",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
        result
    }};
}

fn conv_pool<'g>(x: Tensor<'g>, w: Tensor<'g>, b: Tensor<'g>) -> Tensor<'g> {
    let g = x.graph(); // return the graph to which x belongs
                       //
    let y1 = g.conv2d(x, w, 1, 1) + b;
    let y2 = g.relu(y1);
    x.graph().max_pool2d(y2, 2, 0, 2)
}

fn logits<'g>(x: Tensor<'g>, w: Tensor<'g>, b: Tensor<'g>) -> Tensor<'g> {
    x.graph().matmul(x, w) + b
}

fn inputs(g: &Graph<f32>) -> (Tensor, Tensor) {
    let x = g.placeholder(&[-1, 1, 28, 28]);
    let y = g.placeholder(&[-1, 1]);
    (x, y)
}

fn get_permutation(size: usize) -> Vec<usize> {
    let mut perm: Vec<usize> = (0..size).collect();
    perm.shuffle(&mut rand::thread_rng());
    perm
}

fn main() {
    let ((x_train, y_train), (x_test, y_test)) = mnist::load();
    println!("data loaded!");
    let max_epoch = 5;
    let batch_size = 128isize;
    let num_train_samples = x_train.shape()[0];
    let num_test_samples = x_test.shape()[0];
    let num_batches = num_train_samples / batch_size as usize;

    // convert data shape from [samples, width, height] to [samples, channel, width, height]
    let (x_train, x_test) = (
        x_train
            .into_shape(ndarray::IxDyn(&[num_train_samples, 1, 28, 28]))
            .unwrap(),
        x_test
            .into_shape(ndarray::IxDyn(&[num_test_samples, 1, 28, 28]))
            .unwrap(),
    );

    ag::with(|g| {
        let rng = array::ArrayRng::<f32>::default();
        let w1 = g.variable(rng.random_normal(&[32, 1, 3, 3], 0., 0.1)); // create an ndarray sampled from normal distribution
        let w2 = g.variable(rng.random_normal(&[64, 32, 3, 3], 0., 0.1));
        let w3 = g.variable(rng.glorot_uniform(&[64 * 7 * 7, 10])); // Xavier uniform initialization
        let b1 = g.variable(array::zeros(&[1, 32, 28, 28]));
        let b2 = g.variable(array::zeros(&[1, 64, 14, 14]));
        let b3 = g.variable(array::zeros(&[1, 10]));

        let params = &[w1, w2, w3, b1, b2, b3];

        let params_arrays = params
            .iter()
            .map(|v| v.get_variable_array().unwrap())
            .collect::<Vec<_>>();
        let adam_state = adam::AdamState::new(params_arrays.as_slice());
        let (x, y) = inputs(g);
        let z1 = conv_pool(x, w1, b1);
        let z2 = conv_pool(z1, w2, b2);
        let z3 = g.reshape(z2, &[-1, 64 * 7 * 7]);
        let logits = logits(z3, w3, b3);
        let loss = g.sparse_softmax_cross_entropy(&logits, &y);
        let grads = &g.grad(&[&loss], params);
        let update_ops: &[Tensor] =
            &adam::Adam::default().compute_updates(params, grads, &adam_state, g);
        println!("start train...");
        for epoch in 0..max_epoch {
            println!("start epoch {}", epoch);
            timeit!({
                for i in get_permutation(num_batches) {
                    let i = i as isize * batch_size;
                    let x_batch = x_train.slice(s![i..i + batch_size, .., .., ..]).into_dyn();
                    let y_batch = y_train.slice(s![i..i + batch_size, ..]).into_dyn();
                    g.eval(update_ops, &[x.given(x_batch), y.given(y_batch)]);
                }
            });
            println!("finish epoch {}", epoch);
        }

        let predictions = g.argmax(logits, -1, true);
        let accuracy = g.reduce_mean(&g.equal(predictions, &y), &[0, 1], false);
        println!(
            "test accuracy {:?}",
            accuracy.eval(&[x.given(x_test.view()), y.given(y_test.view())])
        );
    });
}
