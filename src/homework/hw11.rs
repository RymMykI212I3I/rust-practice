use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {

    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
    
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize) {

    data.windows(2)
    
        .enumerate()
        .map(|(i, w)| (i, w[0] + w[1]))
        .min_by_key(|&(_, sum)| sum)
        .map(|(i, _)| (i, i + 1))
        .unwrap()
}

fn print_adjacent_info(data: &[i32]) {

    let (i, j) = min_adjacent_sum(data);


    print!("indexes: ");
    
    for idx in 0..data.len() {
    
        print!("{:>3}.", idx);
    }
    
    println!();

    print!("data:    [");
    
    for (k, v) in data.iter().enumerate() {
        if k < data.len() - 1 {
        
            print!("{:>2}, ", v);
        } else {
        
            print!("{:>2}", v);
        }
    }
    
    println!("]");

    print!("indexes: ");
    
    for k in 0..data.len() {
        if k == i {
        
            print!(" \\__");
            
        } else if k == j {
        
            print!("__/ ");
            
        } else {
        
            print!("    ");
            
        }
    }
    println!();

    println!(
    
        "min adjacent sum={}+{}={} at indexes:{},{}",
        
        data[i],
        data[j],
        data[i] + data[j],
        i,
        j
        
    );
}

fn main() {
    for _ in 0..4 {
    
        let data = gen_random_vector(20);
        print_adjacent_info(&data);
        println!();
        
    }
}
