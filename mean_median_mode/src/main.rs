use std::collections::HashMap;

//calculates mean, median, mode from vector
//assumes vector is not multimodal
fn calculate(myvec: &Vec<i32>) -> (f64, f64, i32) {
    let mut myclone = myvec.clone();
    let median: f64;
    let mut total: i32 = 0;
    let mut mode: Option<i32> = None; 
    let mut values_count = HashMap::new();

    myclone.sort();
    
    if myclone.len() % 2 == 0 {
        median = (myclone[(myclone.len() / 2)] + myclone[(myclone.len() / 2) + 1]) as f64 / 2.0
    } else {
        median = myclone[(myclone.len() / 2) + 1] as f64;
    }
    for i in &myclone {
        total += *i;
        let count = values_count.entry(*i).or_insert(0);
        *count += 1;
    }
    for (key, value) in &values_count {
        if mode == None{
            mode = Some(*key)
        } else {
            if mode.unwrap() < *value {
                mode = Some(*key)
            }
        }
    }

    let mean = total as f64 / myclone.len() as f64;

    return (mean, median, mode.unwrap());
}


fn main() {
    let numbers: Vec<i32> = vec![1, 2, 2, 2, 3, 4];
    let result = calculate(&numbers);
    print!("the mean is {}\n", result.0);
    print!("the median is {}\n", result.1);
    print!("the mode is {}\n", result.2);
    
}
