fn main() {
    println!("Hello, world!");

    another_func(50);

    let x = 5;
    let y = x;
    let z = {
        println!("今ゼットの中にいまーす");
        let x = 2;
        x * y
    };
    println!("{}, {}, {}", x, y, z);

    let number = 6;

    if number <= 6 {
        println!("condition was true")
    } else if number == 5 {
        println!("this number is 5")
    } else {
        println!("condition was false")
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is: {}", number);

    let mut number = 3;

    loop {
        if number <= 0 {
            break;
        }
        println!("{}!", number);
        number = number - 1;
    }
    number = 5;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    println!("for");

    for element in a.iter() {
        println!("The value is: {}", element);
    }
    for element in a.iter().rev() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn another_func(x: i32) {
    println!("This function is another_func");
    println!("The value is: {}", x);
    println!("plus_one: {}", plus_one(x));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
