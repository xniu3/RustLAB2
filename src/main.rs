use crate::Tree::{Node, Empty};

#[derive(Debug)]
#[allow(dead_code)]
enum Tree<T: Ord> {
    Node {
        data: T,
        left_child: Box<Tree<T>>,
        right_child: Box<Tree<T>>,
    },
    Empty,
}
fn insert_node<T:Ord>(TreeNode: Box<Tree<T>>, data: T) -> Box<Tree<T>> {
    return Box::new(
        match(*TreeNode){
            Empty => Node{data:data, left_child: Box::new(Empty), right_child: Box::new(Empty),},
            Node{data:root_data, left_child: left, right_child: right} =>
            if data  < root_data{
                Node{data:root_data, left_child: insert_node(left, data), right_child: right}
            }
            else{
                Node{data:root_data, left_child: left, right_child: insert_node(right, data)}
            }
        }
    );
}


fn main(){
    let mut root = Box::new(Node{data:5, left_child:Box::new(Empty) , right_child: Box::new(Empty)});

    root = insert_node(root, 3);
    root = insert_node(root, 2);
    root = insert_node(root, 4);
    root = insert_node(root, 7);
    root = insert_node(root, 6);
    root = insert_node(root, 8);

    println!("{:#?}",root);
}