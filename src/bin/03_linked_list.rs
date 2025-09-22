struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let mut head: Option<Box<Node>> = None;

    // Manually push_front to build 1 → 2 → 3 → 4 → 5
    head = Some(Box::new(Node {
        data: 5,
        next: head,
    }));
    head = Some(Box::new(Node {
        data: 4,
        next: head,
    }));
    head = Some(Box::new(Node {
        data: 3,
        next: head,
    }));
    head = Some(Box::new(Node {
        data: 2,
        next: head,
    }));
    head = Some(Box::new(Node {
        data: 1,
        next: head,
    }));

    let mut start_node = head.as_ref();    
    while let Some(node) = start_node {
        print!("{} -> ", node.data);
        start_node = node.next.as_ref();
    }

    // updating 3rd element to 30
    let mut start_node = head.as_mut(); 
    let n = 1 ;
    let mut x = 0;
    while let Some(node) = start_node {
        if x == n - 1 {
            node.data = 30;
            break;
        }
        x += 1;
        start_node = node.next.as_mut();
    }

    println!("\nAfter updating 3rd element to 30:");
    let mut start_node = head.as_ref();    
    while let Some(node) = start_node {
        print!("{} -> ", node.data);
        start_node = node.next.as_ref();
    }

    // deleting 3rd element
    let mut start_node = head.as_mut(); 
    let n = 1 ;
    let mut x = 0;
    while let Some(node) = start_node {

        if n == 1 {
            head = head.and_then(|node| node.next);
            break;
            // what is and_then() ?
            // and_then() method is used to chain operations on an Option. 
            // If the Option is Some, it applies a function to the contained value and returns the result. 
            // If the Option is None, it returns None.
        }

        if x == n - 2 {
            if let Some(next_node) = node.next.as_mut() {
                node.next = next_node.next.take(); 
                // what is take() ? 
                // take() method takes the value out of an Option, leaving a None in its place.
            }
            break;
        }
        
        x += 1;
        start_node = node.next.as_mut();
    }

    println!("\nAfter deleting 3rd element ");
    let mut start_node = head.as_ref();    
    while let Some(node) = start_node {
        print!("{} -> ", node.data);
        start_node = node.next.as_ref();
    }
}
