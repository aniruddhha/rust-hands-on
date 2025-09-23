struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn display(head: &Option<Box<Node>>) {
    let mut start_node = head.as_ref();
    print!(" [ ");
    while let Some(node) = start_node {
        start_node = node.next.as_ref();
        print!(" {}", node.data);
        if !node.next.is_none() { print!(" ,") }
    }
    print!(" ] ");
    println!();
}

fn add_nth(head: &mut Option<Box<Node>>, n: usize, data: i32) {
    if n == 1 {
        let old_head = head.take();
        *head = Some(Box::new(Node {
            data,
            next: old_head,
        }));
        return;
    }

    let mut start_node = head.as_mut();
    let mut cnt = 0;

    while let Some(node) = start_node {
        if cnt == n - 1 {
            let old_next = node.next.take(); // tail after insertion point
            node.next = Some(Box::new(Node {
                data,
                next: old_next,
            }));
            return;
        }
        cnt += 1;
        start_node = node.next.as_mut();
    }
}

fn delete_nth(head: &mut Option<Box<Node>>, n: usize) {
    if n == 1 {
        // case deleting first node, change the pointer of head.
        if let Some(mut first) = head.take() {
            *head = first.next.take();
        }
        return;
    }

    let mut start_node = head.as_mut();
    let mut cnt = 0;

    while let Some(node) = start_node {
        if cnt == n - 2 {
            if let Some(mut target) = node.next.take() {
                node.next = target.next.take();
            }
            break;
        }

        cnt += 1;
        start_node = node.next.as_mut();
    }
}

fn update_nth(head: &mut Option<Box<Node>>, n: usize, data: i32) {
    let mut start_node = head.as_mut();

    let mut cnt = 0;
    while let Some(node) = start_node {
        if cnt == n - 1 {
            node.data = data;
        }
        cnt += 1;
        start_node = node.next.as_mut();
    }
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

    add_nth(&mut head, 3, 500);
    display(&head);

    add_nth(&mut head, 1, 900);
    display(&head);

    delete_nth(&mut head, 1);
    display(&head);

    delete_nth(&mut head, 3);
    display(&head);

    delete_nth(&mut head, 5);
    display(&head);

    update_nth(&mut head, 2, 1000);
    display(&head);

    update_nth(&mut head, 1, 9999);
    display(&head);

    update_nth(&mut head, 4, 899);
    display(&head);
}
