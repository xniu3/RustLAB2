#[derive(Debug)]
pub struct TreeNode<'a>{
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>
}

impl <'a> TreeNode <'a>{
    pub fn insert_node(&mut self, newdata: &'a str) {
        if self.data == newdata {
            return
        }
        let new_node = 
            if newdata < self.data { 
                &mut self.left_child 
            } 
            else {
                &mut self.right_child 
            };
        match new_node{
            &mut Some(ref mut node) => node.insert_node(newdata),
            &mut None =>{
                let temp_node = TreeNode{data:newdata, left_child:None , right_child:None};
                *new_node = Some(Box::new(temp_node));
            }
        }
    }
}

fn main(){
    let mut root_val = "4";
    let mut root = TreeNode{data:root_val, left_child: None, right_child:None};
    root.insert_node("2");
    root.insert_node("7");
    root.insert_node("55");
    root.insert_node("52");
    println!("{:#?}",root);
}