struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

trait ListBasic {
    fn insert_nth(&mut self, position: usize, data: i32);
    fn delete_nth(&mut self, position: usize);
    fn update_nth(&mut self, n: usize, data: i32);
    fn display(&self);
    fn reverse(&mut self);
}

impl ListBasic for LinkedList {
    fn insert_nth(&mut self, position: usize, data: i32) {
        if position == 1 {
            let old_head = self.head.take();
            self.head = Some(Box::new(Node {
                data,
                next: old_head,
            }));
            return;
        }

        let mut current = self.head.as_mut();
        let mut count = 0;

        while let Some(node) = current {
            if count == position - 2 {
                let old_next = node.next.take();
                node.next = Some(Box::new(Node {
                    data,
                    next: old_next,
                }));
                return;
            }

            count += 1;
            current = node.next.as_mut();
        }
    }

    fn display(&self) {
        let mut current = self.head.as_ref();

        print!(" [ ");
        while let Some(node) = current {
            print!("{}", node.data);
            current = node.next.as_ref();
            if !node.next.is_none() {
                print!(" ,")
            }
        }
        print!(" ] ");
    }

    fn delete_nth(&mut self, position: usize) {
        let mut current = self.head.as_mut();
        let mut cnt = 1;

        if position == 1 {
            if let Some(first) = current.as_mut() {
                self.head = first.next.take();
                return;
            }
        }

        while let Some(node) = current {
            if cnt == position - 1 {
                let target = node.next.as_mut();
                if let Some(tg) = target {
                    node.next = tg.next.take();
                }
                return;
            }

            cnt += 1;
            current = node.next.as_mut();
        }
    }
    // fn delete_nth(&mut self, n: usize) {

    fn update_nth(&mut self, position: usize, data: i32) {
        let mut current = self.head.as_mut();
        let mut cnt = 1;

        while let Some(node) = current {
            if cnt == position {
                node.data = data;
                return;
            }

            cnt += 1;
            current = node.next.as_mut();
        }
    }

    fn reverse(&mut self) {
        // The part we’ve already reversed
        let mut reversed: Option<Box<Node>> = None;

        // The part we still need to process (take ownership of head)
        let mut current: Option<Box<Node>> = self.head.take();

        // Process nodes one by one
        while let Some(mut current_node) = current {
            // 1) Remember the rest of the list
            let next_unprocessed = current_node.next.take();

            // 2) Point current node back to the reversed prefix
            current_node.next = reversed;

            // 3) Grow the reversed prefix to include current node
            reversed = Some(current_node);

            // 4) Continue with the rest
            current = next_unprocessed;
        }

        // Install the new head (fully reversed list)
        self.head = reversed;
    }
}

fn main() {
    let mut list = LinkedList { head: None };

    list.insert_nth(1, 10);
    list.insert_nth(2, 20);
    list.insert_nth(3, 30);
    list.display();
    // list.insert_nth(2, 15);
    // list.insert_nth(1, 5);

    list.delete_nth(3);
    list.display();

    list.delete_nth(1);
    list.display();

    list.update_nth(1, 999);
    list.display();

     list.insert_nth(2, 399);
    list.insert_nth(3, 455);
     list.display();


      list.update_nth(1, 222);
       list.display();

       list.update_nth(3, 444);
       list.display();

       list.reverse();
         list.display();
}


