mod arrays;
mod linked_list;
fn main(){ 
    let arr = [1, 2, 3, 4];
    func(&arr);
    let str_arr = ["string", "is", "me", "for"];
    linked_list::string_arr(&str_arr);
    linked_list::linked_list(&arr);
}
