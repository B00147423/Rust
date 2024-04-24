fn main(){
    let mut x = 10;
    let x_ptr: *mut i32 = &mut x;

    unsafe {
        *x_ptr = 20;
    }
    println!("x after modification: {}", x);
}