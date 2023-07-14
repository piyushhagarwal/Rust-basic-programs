#[allow(dead_code)]
#[derive(Debug)]
struct Node <T>{
    value : T,                      
    next : Option<Box<Node<T>>>
    //next stores a pointer to next node.
    //Box is used to store the pointer of type Node<T> 
    //Box is wrapped in Option because it can have either "Some" value or "None".
}

#[allow(dead_code)]
impl<T> Node<T> {
    //This function creates a new node 
    fn new(value : T) -> Node<T> {
        Node 
        {
            value,       // This is a short form of value:value
            next : None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node1 = Node::new(20);
        assert_eq!(node1.value,20)
    }
}
