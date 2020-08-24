extern crate serde;
#[macro_use]
extern crate serde_derive;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use utils::dataset::get_boston_record_from_file;
use rusty_machine::linalg::Matrix;
use rusty_machine;
use rusty_machine::linalg::Vector;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filepath = "data/boston.csv";
    let mut data = get_boston_record_from_file(&filepath);
    data.shuffle(&mut thread_rng());
    let test_size: f64 = data.len() as f64 * 0.2;
    let test_size = test_size.round() as usize;

    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();

    let X_train =: Vec<f64> = train_data.iter().flat_map(|r| r.into_feature_vector())
    .collect();
    let y_train: Vec<f64> = train_data.iter().map(|r| r.into_target()).collect();

    let X_test: Vec<f64> = test_data.iter().flat_map(|r| r.into_feature_vector()).collect();
    let y_test: Vec<f64> = test_data.iter().map(|r| r.into_target()).collect();

    let X_train = Matrix::new(train_size, 13, X_train);
    let y_train = Vector::new(train_size, 1, y_train);

    let X_test = Matrix::new(test_size, 13, X_test);
    let y_test = Matrix::new(test_size, 1, y_test);
    

}
