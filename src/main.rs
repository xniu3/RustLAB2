#[derive(Debug)]
pub struct TreeNode<'a>{
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>
}

impl <'a> TreeNode <'a>{
    pub fn insert_node(&mut self, data: &'a str) {
        if self.data == data {
            return
        }
        let new_node = 
            if data < self.data { 
                &mut self.left_child 
            } 
            else {
                &mut self.right_child 
            };
        match new_node{
            &mut Some(ref mut node) => node.insert_node(data),
            &mut None =>{
                let temp_node = TreeNode{data:data, left_child:None , right_child:None};
                *new_node = Some(Box::new(temp_node));
            }
        }
    }
}

fn main(){
    let mut root_val = "55";
    let mut root = TreeNode{data:root_val, left_child: None, right_child:None};
    root.insert_node("6");
    root.insert_node("9");
    root.insert_node("701");
    root.insert_node("705");
    println!("{:#?}",root);
}