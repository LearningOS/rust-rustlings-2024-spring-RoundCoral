// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    let mut data_char = get_char(&data).to_string(); //将借用得转化为string并绑定到可变变量data_char上

    string_uppercase(data_char);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap() //借用的值返回
}

// Should take ownership
fn string_uppercase(mut data:String) {
    data = data.to_uppercase();

    println!("{}", data);
}
