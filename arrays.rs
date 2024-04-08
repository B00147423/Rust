fn func(arr: &[i32]) {
    for &item in arr {
        print!("{}", item);
    }
}


fn string_arr(arr: &[&str]){
    for &item in arr{
        print!("{} ", item);
    }
    println!();
}

fn main(){
    let arr = [1, 2, 3, 4];
    func(&arr);
    let str_arr = ["string,", "array", "list, is", "this"];
    string_arr(&str_arr);
}
