// use std::collections::HashMap;

// fn main() {
//     let vector = vec![2, 4, 5, 99, 99, 4, 4];
//     let result = get_median(&vector);

//     println!("res = {:#?}", result);
// }

// fn get_median(v: &Vec<i32>) -> HashMap<String, f32> {
//     let mut result = HashMap::new();
//     let mut total = 0;

//     let mut occurency = HashMap::new();

//     for i in v {
//         total += i;
//         let counter = occurency.entry(i).or_insert(0);
//         *counter += 1;
//     }
//     let size = v.len() as i32;
//     println!("total = {}, size = {}", total, size);
//     let mean = total as f32 / size as f32;
//     result.insert(String::from("mean"), mean);
//     let mut a = v.clone();
//     a.sort();
//     let median: f32;
//     if a.len() % 2 == 0 {
//         median = (a[a.len() / 2 - 1] + a[a.len() / 2]) as f32 / 2.0;
//     } else {
//         median = a[a.len() / 2] as f32;
//     }
//     result.insert(String::from("median"), median);
//     let mut mod_value: i32 = 0;
//     let mut mod_key: f32 = 0.0;
//     for (key, val) in occurency.iter() {
//         if val > &mod_value {
//             mod_value = *val;
//             mod_key = **key as f32;
//         }
//     }
//     result.insert(String::from("mod"), mod_key);

//     result
// }

use std::vec;

fn main() {
    tribonacci(&[1.0, 2.0, 3.0], 5);
}

fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    if n <= 3 {
        let a = signature.to_vec();
        return a;
    };

    let mut res = signature.to_vec();

    for i in 3..n {
        res.push(res[i - 1] + res[i - 2] + res[i - 3]);
    }

    return vec![1.0];
}
