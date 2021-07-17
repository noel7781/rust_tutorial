use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, Option<u32>>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => v.unwrap(),
            None => {
                let v = (self.calculation) (arg);
                self.value.insert(v, Some(v));
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("시간이 오래 걸리는 계산을 수행중...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔굽혀펴기를 하세요",
            expensive_result.value(intensity)
            );
        println!(
            "다음에는 {}번의 윗몸 일으키기를 하세요!",
            expensive_result.value(intensity)
            )
    } else {
        if random_number == 3 {
            println!("오늘은 충분히 쉬세요");
        } else {
            println!("오늘은 {}분간 달리기를 하세요!",
            expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}
