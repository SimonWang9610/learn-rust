struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let v = command.as_bytes();
        let mut i = false;
        let mut result: Vec<u8> = vec![];

        for &ele in v.iter() {

            match ele {
                71 => result.push(71),
                40 => i = true,
                41 => {

                    if i {
                        result.push(111);
                    } else {
                        result.push(97);
                        result.push(108);
                    }
                },
                _ => i = false,
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// Error Reason: String::as_bytes() method return the number based on Decimal