#[allow(dead_code)]
#[derive(Debug)]

//Structure for Node
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

//Structure for linked list which consists of "head" and "length"
struct Linked_List<T> {
    head : Option<Box<Node<T>>>,
    length : u32
}

impl<T> Linked_List<T> {
    
    //Creates an empty Linked List
    fn new() -> Linked_List<T>{
        Linked_List 
        { 
            head: None,
            length: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node1 = Node::new(20);
        let ll: Linked_List<i32> = Linked_List::new();
        assert_eq!(ll.length,0)
    }
}
