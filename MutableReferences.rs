fn add_five(num: &mut i32){
    *num += 5;
}

fn main(){
    let mut x = 10;
    add_five(&mut x);
    println!(" x after addign 5: {}", x);
}

