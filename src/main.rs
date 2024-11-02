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

fn myprint<T: std::fmt::Display>(msg: &T) {
    // println!("{}", msg);
    println!("{}", *msg);
}

// fn dummy_func() {
//     let x;
//     {
//         let y = 1;
//         x = &y;
//     }
//     println!("{}", x);
// }


fn myclear(x: &mut String) {
    x.clear();
}

fn mut_clear() {
    let mut s = "Hello".to_string();
    println!("s= {}", s);

    let s_ref = &mut s;
    myclear(s_ref);
    println!("s= {}", s);
}

fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'a [i32], &'b [i32]) {
    (&x[..end], &y[..end])
}

fn array_fn() {
    let ary = [1, 2, 3, 4, 5];
    for aa in &ary {
        println!("{}", aa);
    }
    println!("ary[1] = {}", ary[1]);
}

fn tuple_fn() {
    let tup = (1, 2.5, true, "Hello");
    println!("{}, {}, {}, {}", tup.0, tup.1, tup.2, tup.3);
}

fn slice_fn() {
    let ary = [1, 2, 3, 4, 5];
    let ary_sliced = &ary[0..2];
    for aa in ary_sliced {
        println!("{}", aa);
    }
}

fn vector_fn() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("before push: {:?}", v);
    v.push(10);
    println!("after push: {:?}", v);
    v[2] += 10;
    println!("after add: {:?}", v);
    println!("&v[3..] = {:?}", &v[3..]);
}

fn call_functions() {
    let s = "Hello".to_string();
    // let ss = s.clone();
    myprint(&s);
    myprint(&s);

    let t = "Yeah".to_string();
    let t_ref = &t;
    myprint(t_ref);
    myprint(t_ref);

    {
        let v1 = [1, 2, 3, 4, 5];
        let p = pick1(&v1, 2);
        for ss in p {
            println!("{}", ss);
        }
    }
    {
        let v1 = [1, 2, 3, 4, 5];
        let v2 = [6, 7, 8];
        let p = pick2(&v1, &v2, 2);
        for ss in p.0 {
            println!("{}", ss);
        };
        for ss in p.1 {
            println!("{}", ss);
        };
    }
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
    // module_hello::print_hello();
    // copy_semantics();
    // move_semantics();
    // call_functions();
    // mut_clear()
    // array_fn();
    tuple_fn();
    slice_fn();
    vector_fn();
}
