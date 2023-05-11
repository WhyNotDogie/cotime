use cotime;
use quote;

cotime::cotime!{{
    std::env::set_var("ABC", "9");
    let a: i32 = 5+8+1+std::env::var("ABC").unwrap().parse::<i32>().unwrap();
    a
}}