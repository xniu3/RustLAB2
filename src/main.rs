#[derive(Debug)]
struct TreeNode<'a>{
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>
}


fn main(){
    let mut root_val = "1";
    let mut root = TreeNode{data:root_val, left_child: None, right_child:None};
    
}