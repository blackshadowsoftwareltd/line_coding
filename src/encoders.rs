pub fn unipolar_nrz(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|bit| match bit {
            '1' => 1,
            '0' => 0,
            _ => panic!("Invalid input: only binary characters ('0' and '1') are allowed."),
        })
        .collect()
}

pub fn polar_nrz(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|bit| match bit {
            '1' => 1,  // Binary '1' -> Positive voltage
            '0' => -1, // Binary '0' -> Negative voltage
            _ => panic!("Invalid input! Only binary characters ('0' and '1') are allowed."),
        })
        .collect()
}

pub fn manchester(input: &str) -> Vec<i32> {
    input
        .chars()
        .flat_map(|bit| match bit {
            '1' => vec![-1, 1], // Binary '1' -> Low-to-high transition
            '0' => vec![1, -1], // Binary '0' -> High-to-low transition
            _ => panic!("Invalid input! Only binary characters ('0' and '1') are allowed."),
        })
        .collect() // Flatten the nested vectors into a single vector
}

pub fn ami(input: &str) -> Vec<i32> {
    let mut last_polarity = 1; // Keeps track of the last polarity used for '1'
    input
        .chars()
        .map(|bit| match bit {
            '1' => {
                let current = last_polarity;
                last_polarity *= -1; // Alternate polarity for the next '1'
                current
            }
            '0' => 0, // Binary '0' -> No voltage
            _ => panic!("Invalid input! Only binary characters ('0' and '1') are allowed."),
        })
        .collect()
}
