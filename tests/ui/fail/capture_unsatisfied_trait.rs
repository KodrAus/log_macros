#![feature(stmt_expr_attributes, proc_macro_hygiene)]

#[macro_use]
extern crate log_macros;

// Does not implement `Display`
struct Input;

fn main() {
    call(Input);
}

fn call(input: Input) {

    let kvs: &[(&str, log_macros_private::__private::Value)] = &[
        #[display] input,
    ];

    println!("{:?}", kvs);
}
