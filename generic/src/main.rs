fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("{}", result);
    
}

fn largest<T: PartialOrd + Copy> (list:&[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if largest < item  {
            largest = item;
        }
    }
    largest
}
