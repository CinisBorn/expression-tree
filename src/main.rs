use infix_to_postfix::convert;
use expression_tree::Node;

fn main() {
    let infix = convert("1 + 1 * ( 2 / 2 )");
    let test = vec!["a", "b", "+", "c", "d", "e", "+", "*", "*"].iter().map(|e| e.to_string()).collect();
    let tree = Node::new().build(test);
    
    dbg!("{:?}", tree);
}
