#[derive(Debug)]
struct TreeNode<'a>{
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>
}


fn main(){
    
}