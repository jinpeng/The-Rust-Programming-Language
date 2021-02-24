use std::collections::HashMap;

fn mean(v: &Vec<i32>) -> f32 {
    v.iter().sum::<i32>() as f32 / v.len() as f32
}

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    *v.get(v.len() / 2).unwrap()
}

fn mode(v: &Vec<i32>) -> Option<i32> {
    let mut counts = HashMap::new();

    v.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}

fn main() {
    let mut vec: Vec<i32> = vec![42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];
    let mean = mean(&vec);
    let median = median(&mut vec);
    let mode = mode(&vec).unwrap();
    println!("{:?}", vec);
    println!("Mean: {}, median: {}, mode: {}", mean, median, mode);
}
