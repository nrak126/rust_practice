use std::io;

fn main() {
    // println!("F or C ?");
    // let mut c: char;
    // io::stdin().read_line(&mut c).expect("Failed to read line");

    // println!("how much temparature?");
    let mut temparature: f64 = 50.0;
    // io::stdin()
    //     .read_line(&mut temparature)
    //     .expect("Failed to read line");

    // if c == 'F' {
        println!("F({}) -> C({})", temparature, f_to_c(temparature));
    // }
    // if c == 'C' {
        println!("C({}) -> F({})", temparature, c_to_f(temparature));
    // }
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f64) -> f64 {
    c / 5.0 * 9.0 + 32.0
}
