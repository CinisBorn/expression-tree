#[derive(Debug)]
pub enum StackError {
    EmptyStack 
}

#[derive(Debug)]
pub struct Node {
    element: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

#[derive(Debug)]
pub struct Stack {
    elements: Vec<Node> 
}

impl Stack{
    pub fn new() -> Self {
        Self {
            elements: Vec::new() 
        }
    }
    
    pub fn pop_two(&mut self) -> Result<(Node, Node), StackError> {
        if self.elements.len() < 2 {
            return Err(StackError::EmptyStack)
        }
        
        let t1 = self.elements.pop().unwrap();
        let t2 = self.elements.pop().unwrap();
        
        Ok((t1, t2))
    }
    
    pub fn push(&mut self, value: Node) {
        self.elements.push(value);
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            element: String::new(),
            left: None,
            right: None
        }
    }
    
    pub fn build(&mut self, expression: Vec<String>) -> Stack {
        let mut stack: Stack = Stack::new();
        let operators: [String; 4] = [
            String::from("+"), 
            String::from("-"), 
            String::from("*"), 
            String::from("/")
        ];
        
        for e in expression {
            if operators.contains(&e) {
                let (t1, t2) = stack.pop_two().expect("A Non Empty Vec");
                
                stack.push(Node { 
                    element: e, 
                    left: Some(Box::new(t1)), 
                    right: Some(Box::new(t2))
                });
                
                continue 
            }
            
            stack.push(Node { element: e, left: None, right: None });
        }
        
        stack
    }
}