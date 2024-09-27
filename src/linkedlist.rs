// Linked list "Class"
// Michael Rizig
// 9/26/24


// Struct to hold attributes of Node
pub struct Node {
    pub data: i32,
    // Option = Can be Some or None
    // Box = pointer (created in heap)
    pub next: Option<Box<Node>>,
    pub initilized:bool
}
// implimentation to store functions of node (constructor)
impl Node {
    // returns box pointer to Node
    pub fn new(data: i32) -> Box<Node> {
        Box::new(Node { data, next: None , initilized:false})
    }
}
// struct to hold attributes of linkedlist (head)
pub struct LinkedList {
    // Option = Can be Some or None
    // Box = pointer (created in heap)
    pub head: Option<Box<Node>,>,
    pub nodes: Vec<Box<Node>>
}
// implimnetation to store functions of linked list
impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: Some(Node::new(00)) , nodes: Vec::new()}
    }
    // function to insert value to end of list
    pub fn insert(&mut self, data: i32) {
        // check if head is nulll
        match self.head {
            // if it is, create new node with the param data
            None => {
                self.nodes = Vec::new();
                self.nodes.push(Node::new(data));
                self.head = self.nodes[0];
                self.head.initilized = true;
            }
            // else head has some value
            Some(_) => {
                // create current pointer to itterate through the list, set it equal to heads memory value
                let mut current = self.nodes[0];
                // while it has some value, current = current.next
                while let Some(ref mut node) = *current {
                    current = &mut node.next;
                }
                // finally when loop terminates, current is null, so make current the new value
                *current = Some(Node::new(data));
            }
        }
    }
    //function to delete node
    pub fn delete(&mut self, data: i32){
        // if list is empty, print and return.
        match self.head {
            None => {
                println!("List is Empty!");
                return;
            }
            // else list is not empty
            Some(_) => {
                // set current and prev pointers:
                // Nodes are Option<Box<Node>>, so pointer to node is &mut Option<Box<Node>>
                let mut current: &mut Option<Box<Node>> = &mut self.head;
                let mut prev: &mut  Option< Box<Node>> =  &mut None;
                // loop through list until value of data is found or we reach the end
                while let Some(ref mut node) = current {
                    // if we find the data:
                    if node.data == data {
                        // check prev pointer to determine course of action
                        match prev {
                            
                            // if prev pointer is still none, our data is in the head variable. we simply need to move head to next variable
                            None => {
                                // current points to our head value
                                // .take() replaces prev value with None
                                // example x=5; y=x.take(); y now equals 5 and x now equals None;
                                println!("Node.next= {}", node.next.as_mut().unwrap().data );
                                self.head = self.head.as_mut().unwrap().next.take();
                            }
                            // update prev.next to current.next (deletng current)
                            Some(prev)=>{
                                prev.next = node.next.take();

                            }
                        }
                        return;
                    }
                    prev = &mut current;
                    current = &mut node.next;
                }
                // if we make it through the loop, no value matched,
                return
            }
        }
    }
}
