fn main() {
    let reference_to_nothing = dangle();

    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &s;
    // println!("{} and {}", r1, r2);
    
    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;

    change(&mut s);

    let len = calculate_length(&s);

    println!("'{}'의 길이는 {}입니다.", s, len);
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// error
// fn change(s: &String) {
//     s.push_str(", world!");
// }