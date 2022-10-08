
#[derive(Debug)]

pub struct TreeNode<T> {
    pub data: T,
    pub left_child: Option<Box<TreeNode<T>>>,
    pub right_child: Option<Box<TreeNode<T>>>,
}

impl TreeNode<i32>{
    
    pub fn insert_node(&mut self, data:i32)  {

        if self.data == data {
            return
        }

        let new_node = if data < self.data { 
            &mut self.left_child
        } 
        
        else {
            &mut self.right_child 
        };
        match new_node {
            Some(ref mut node) => node.insert_node(data),
            None => {
                let node = TreeNode{data, left_child:None, right_child:None};
                let box_node = Some(Box::new(node));
                *new_node = box_node;
            },
        }
    }
}

fn main(){
    let mut root = TreeNode{
        data: 5,
        left_child: None,
        right_child: None, 
    };
    root.insert_node(4);
}

