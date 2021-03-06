extern crate ndarray;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

type Ndarray = ndarray::Array<f32, ndarray::IxDyn>;

pub fn load() -> ((Ndarray, Ndarray), (Ndarray, Ndarray)) {
    println!("current directory {:?}", env::current_dir().unwrap());
    let (train_x, num_image_train): (Vec<f32>, usize) =
        load_images("../data/mnist/train-images.idx3-ubyte");
    let (train_y, num_label_train): (Vec<f32>, usize) =
        load_labels("../data/mnist/train-labels.idx1-ubyte");

    let (test_x, num_image_test): (Vec<f32>, usize) =
        load_images("../data/mnist/t10k-images.idx3-ubyte");

    let (test_y, num_label_test): (Vec<f32>, usize) =
        load_labels("../data/mnist/t10k-labels.idx1-ubyte");

    let as_arr = Ndarray::from_shape_vec; // create am array withe the given shape from a vector
    let x_train = as_arr(ndarray::IxDyn(&[num_image_train, 28 * 28]), train_x).unwrap();
    let y_train = as_arr(ndarray::IxDyn(&[num_label_train, 1]), train_y).unwrap();
    let x_test = as_arr(ndarray::IxDyn(&[num_image_test, 28 * 28]), test_x).unwrap();
    let y_test = as_arr(ndarray::IxDyn(&[num_label_test, 1]), test_y).unwrap();
    ((x_train, y_train), (x_test, y_test))
}

fn load_images<P: AsRef<Path>>(path: P) -> (Vec<f32>, usize) {
    let file = File::open(path).expect("please sure the data file exists");
    let ref mut buf_reader = io::BufReader::new(file);
    let magic = read_be_u32(buf_reader);

    if magic != 2051 {
        panic!("Invalid magic number, expect 2051, got {}", magic)
    }

    let num_image = read_be_u32(buf_reader) as usize;
    let rows = read_be_u32(buf_reader) as usize;
    let cols = read_be_u32(buf_reader) as usize;

    assert!(rows == 28 && cols == 28);

    let mut buf: Vec<u8> = vec![0 as u8; num_image * rows * cols];
    let _ = buf_reader.read_exact(buf.as_mut());
    let ret: Vec<f32> = buf.into_iter().map(|x| (x as f32) / 255.).collect();
    (ret, num_image)
}

fn load_labels<P: AsRef<Path>>(path: P) -> (Vec<f32>, usize) {
    let ref mut buf_reader =
        io::BufReader::new(File::open(path).expect("please sure label data exists"));
    let magic = read_be_u32(buf_reader);

    if magic != 2049 {
        panic!("invalid magic number, expect 2049, got {}", magic)
    }

    let num_label = read_be_u32(buf_reader) as usize;
    let mut buf: Vec<u8> = vec![0 as u8; num_label];
    let _ = buf_reader.read_exact(buf.as_mut());
    let ret: Vec<f32> = buf.into_iter().map(|x| x as f32).collect();
    (ret, num_label)
}

fn read_be_u32<T: Read>(reader: &mut T) -> u32 {
    let mut buf = [0 as u8; 4];
    let _ = reader.read_exact(&mut buf);
    u32::from_be_bytes(buf)
}

fn read_file<P: AsRef<Path>>(path: P) {
    let test = File::open(path).expect("fail to open file");
    let ref mut buf_reader = io::BufReader::new(test);
    let magic = read_be_u32(buf_reader);
    println!("magic {}", magic);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn open_file() {
        load();
    }
}
