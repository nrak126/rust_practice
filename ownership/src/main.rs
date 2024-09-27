fn main() {
    // 文字列リテラル
    // スタック領域
    // {
    //     let s = "hello";
    //     println!("{}", s);
    // }
    // println!("{}", s);

    // 文字列型
    // ヒープ領域
    // {
    //     let mut s = String::from("hello");
    //     s.push_str(", world!");
    //     println!("{}", s);
    // }
    // println!("{}", s);

    // スタック型
    // let x = 5;
    // yにコピー
    // let y = x;

    // ヒープ領域にある"hello"のポインタに束縛される
    // let s1 = String::from("hello");

    // s1をs2にコピー
    // s1は所有権を失う
    // 所有権を失うと使えなくなる
    // let s2 = s1;

    // s1をまだ使いたいけどs2にコピーしなければならない時
    // let s2 = s1.clone();

    // println!("{}", s1);
    // println!("{}", s2);

    // let s = String::from("hello");
    // sの所有権が関数の引数に移動する
    // takes_ownership(s);
    // こうすれば使える
    // takes_ownership(s.clone());

    //sの所有権がないから使えない
    // println!("{}", s);

    // let x = 5;
    // i32型はヒープ領域を使わないので所有権関係なし
    // makes_copy(x);

    // let s1 = gives_ownership();

    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is '{}'.", s1, len);


    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", s);

    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;
    // let r3 = &mut s;

    // どこかで可変な参照をするとき（mutを使う時）そこ以外で参照ができない

    let r1 = & s;

    {
        let r2 = & s;
    }
    
    let r2 = &mut s;

    // スコープで区切られてたら参照できる！

    println!("{}{}{}", s, r1, r2);
}

fn calculate_length(s: &String) -> usize {
    //sは所有権を持たない
    //sは参照してるだけ
    //変数の参照を利用することを借用という
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // 戻り値でも所有権が移動する
    some_string
    // もし戻り値が無ければsome_stringの所有権は破棄され、ヒープ上のhelloが消える
    // このままだとhelloがヒープ上に残ったまま
}