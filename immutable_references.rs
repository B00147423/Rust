fn print_elements(v: &[i32]){
    for &item in v {
        println!("{}", item);
    }
}

fn main(){
    let vec = vec![1, 2, 3 ,4];
    print_elements(&vec);
}
