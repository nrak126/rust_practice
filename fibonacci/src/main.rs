use std::collections::HashMap;

fn main() {
    let mut cache: HashMap<u32, u32> = HashMap::new();

    cache.insert(0, 0);
    cache.insert(1, 1);

    let x: u32 = 40;
    println!("fibonacci({}) = {}", x, fibonacci(x, &mut cache));
}

fn fibonacci(n: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    if let Some(&value) = cache.get(&n) {
        return value;
    }

    let result = fibonacci(n - 1, cache) + fibonacci(n - 2, cache);
    cache.insert(n, result);

    result
}
