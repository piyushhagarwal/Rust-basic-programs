struct Queue<T>{
    items: Vec<T>
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { items: Vec::new() }
    }

    fn push(&mut self, element : T){
        self.items.push(element);
    }

    fn pop(&mut self) -> Option<T>{
        match self.items.is_empty() {
            true => None,
            false => Some(self.items.remove(0))
        }
    }

    fn front(&self) -> Option<&T> {
        self.items.get(0)
    }

    fn is_empty(&self) -> bool{
        self.items.is_empty()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}


fn main() {
    let mut queue = Queue::new();

    // Enqueue elements
    queue.push("apple".to_string());
    queue.push("banana".to_string());
    queue.push("orange".to_string());

    // Check if the queue is empty
    if queue.is_empty() {
        println!("Queue is empty!");
    } else {
        println!("Queue is not empty!");
    }

    //Pop the element 
    queue.pop().expect("The queue is empty");

    //Get the front element of the pass
    let front_element = queue.front().expect("Queue is not empty");
    println!("The front element of the queue is {}",front_element);

    // Check the length of the queue
    println!("Queue length: {}", queue.len()); 
}
