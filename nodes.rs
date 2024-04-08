struct Node{
    bit= i32,
    priority: u32,
    left_child: &Node,
    right_child: &Node,
}


fn post_order_traversal(node: &Node)
{
    if let Some(left) = node.left_child{
        post_order_traversal(&left);
        
    }
    if let Some(right) = node.right_child{
        post_order_traversal(&right);
    }
    display(node);
}

//pre order traversal
fn pre_order_traversal(node:&Node){
    display(node);
    if let Some(left) = node.left_child{
        
        pre_order_traversal(&left);
    }
    if let Some(right) = node.right_child{
        pre_order_traversal(&right);
    }

}


//in order traversal
fn in_order_traversal(node: &Node){ 
    if let Some(left) = node.left_child{ // let sum = if it's not null and there is left child node
        in_order_traversal(left);
    } 
    display(node);
    if let Some(left) = node.left_child{ // let sum = if it's not null and there is right child node
        in_order_traversal(node.right); 
    } 
};

fn dislay(node &Node){
    print!("bit: {}, priority: {}, left_child:{}, right_child: {}", node.bit, node.priority, node.left_child, node.right_child);
}

fn main(){

    let root = Node
    post_order_traversal(&root);
    pre_order_traversal();
    in_order_traversal();
}
