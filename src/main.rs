mod module_hello;
fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans { // ansがSomeの場合、その中の値をxに束縛する
        println!("{}", x);
    } else {
        println!("None");
    }
}

fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}

fn copy_semantics() {
    let s = 1;
    let t = s;
    println!("{}", t);
    println!("{}", s);
}

fn move_semantics() {
    let s = "Hello".to_string();
    println!("{}", s);
    let t = s;
    println!("{}", t);
    // println!("{}", s); 変数sは "Hello" 文字列の値を格納したメモリ領域への所有権を失っており、他の値の所有権を持っていないため、コンパイルエラーとなる
}

fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

fn call_functions() {
    let s = "Hello".to_string();
    let ss = s.clone();
    myprint(s);
    myprint(ss);
}

fn main() {
    // call all functions
    // func_ex_print_some(func_ex_div_some(10, 5));
    // func_ex_print_some(func_ex_div_some(10, 0));
    // func_ex_print_some_match(func_ex_div_some(10, 5));
    // func_ex_print_some_match(func_ex_div_some(10, 0));
    // func_ex_print_result(func_ex_div_result(10, 5));
    // func_ex_print_result(func_ex_div_result(10, 0));
    // call module function
    module_hello::print_hello();
    copy_semantics();
    move_semantics();
    call_functions();
}
