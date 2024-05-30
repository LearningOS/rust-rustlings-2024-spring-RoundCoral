// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // <_>使编译器自己推断类型
    let mut shopping_list: Vec<_> = Vec::new();
    shopping_list.push("milk");
}
