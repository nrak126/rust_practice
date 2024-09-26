fn main() {
    let x: u32 = 6;
    println!("fibonatch({}) = {}", x, fibonatch(x));
}

fn fibonatch(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonatch(n - 1) + fibonatch(n - 2)
    }
}
