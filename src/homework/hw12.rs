fn count_permutation(shipments: &Vec<u32>) -> isize {

    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;


    if total % n != 0 {
        return -1;
    }

    let avg = total / n;
    let mut moves = 0isize;
    let mut imbalance = 0isize;

    for &load in shipments.iter() {
        imbalance += load as isize - avg as isize;
        moves += imbalance.abs();
    }

    moves
}


fn gen_shipments(n: usize) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut vec: Vec<u32> = (0..n).map(|_| rng.gen_range(10..100)).collect();

    
    let total: u32 = vec.iter().sum();
    let remainder = total as usize % n;

    if remainder != 0 {
        
        if vec[n - 1] >= remainder as u32 {
            vec[n - 1] -= remainder as u32;
        } else {
            vec[n - 1] += (n - remainder) as u32;
        }
    }

    vec
}


fn main() {

    let input = vec![1, 1, 1, 1, 6];
    let result = count_permutation(&input);
    println!("Result: {}", result);

    let test = vec![8, 2, 2, 4, 4];
    println!("Moves for {:?} = {}", test, count_permutation(&test));

    let valid = gen_shipments(10);
    println!("Generated balanced shipments: {:?}", valid);
    println!("Result: {}", count_permutation(&valid));
    
}
