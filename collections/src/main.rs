use std::collections::HashMap;

fn main() {
    one();
    two();
    three();
}

fn one() {
    let mut v = vec![1,10,2,9,3,8,4,2,2,2,4,4,4,8,7,7,6,7,4,6];
    v.sort();
    println!("{:?}", v);
    let mut sum = 0;
    for x in &v {
        sum += x;
    }
    println!("sum : {}", sum);
    let length = v.len();
    println!("mid : {}", v[length/2]);

    let mut map = HashMap::new();
    let mut max_count = 0;
    let mut mode = -1;
    for x in &v {
        let count = map.entry(x).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = *x;
        }
    }
    println!("mode : {}", mode);
}

fn two() {
    let s1 = "first";
    let s2 = "apple";

    pig_latin(s1);
    pig_latin(s2);

}

fn pig_latin(s: &str) {
    let vowel = vec!["a", "e", "i", "o", "u"];

    let first = &s[0..1];
    let mut exist = false;
    for i in &vowel {
        if *i == first {
            exist = true;
        }
    }

    if exist {
        println!("{}-hay", s);
    } else {
        println!("{}-{}ay", &s[1..], first);
    }

}

fn three() {

}
