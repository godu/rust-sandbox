#[path = "enum.rs"]
mod enum_;
mod function;
mod guessing;
mod method;
mod string;
#[path = "struct.rs"]
mod struct_;
mod vector;

fn main() {
    guessing::main();
    function::main();
    string::main();
    struct_::main();
    method::main();
    enum_::main();
    vector::main();
}
