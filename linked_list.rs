use std::collections::LinkedList;


//linekd list for 
fn linked_list(arr: &[i32]){
    let mut list = LinkedList::new(); //
    for &item in arr{
        list.push_back(item as u32);
        print!("{} ", item);
    }    
}

fn linked_list_string(arr: &[&str]){
    let mut list = LinkedList::new(); //
    for &item in arr{
        list.push_back(item);
        println!("{} ", item);
    }
}


fn main(){

    let arr = [1, 2, 3, 4];
    let str_arr = ["string", "is", "me", "for"];
    linked_list(&arr);
    linked_list_string(&str_arr);
}
