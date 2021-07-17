use std::io;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};

fn main() {
    let mut rng = rand::thread_rng();
    let random_number:i32 = rng.gen_range(1..101);
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let number:i32 = s.trim().parse().unwrap();
        println!("{}", number);

        //println!("random number: {}", random_number);
        match number.cmp(&random_number) {
            Ordering::Less => println!("Smaller"),
            Ordering::Greater => println!("Bigger"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
        }
    }
}
