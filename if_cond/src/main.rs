// use std::fs::read;

fn main() {
    let n:i32 = -4;

    if n <= 5 && n>0 {println!("Num is less than 5");}
    else if n <= 10 && n >5 {println!("Num is less than 10");}
    else {println!("Num is not within range");}

    let n:i32 = 5;
    println!("{}",
        if n <= 5 && n>0 {"Num is less than 5"}
        else if n <= 10 && n >5 {"Num is less than 10"}
        else {"Num is not within range"}
    )
}
