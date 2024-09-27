// Linked list "Class"
// Michael Rizig
// 9/26/24



// Struct to hold attributes of Node
pub struct Node {
    pub data: i32,
    // Option = Can be Some or None
    // Box = pointer (created in heap)
    pub next: Option<i32>,
    
}
// implimentation to store functions of node (constructor)
impl Node {
    // returns box pointer to Node
    pub fn new(data: i32) -> Box<Node> {
        Box::new(Node { data, next: None })
    }
}
// struct to hold attributes of linkedlist (head)
pub struct LinkedList {
    // Option = Can be Some or None
    // Box = pointer (created in heap)
    pub head: Option<i32>,
    pub nodes: Vec<Node>
}
// implimnetation to store functions of linked list
impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None , nodes: Vec::new()}
    }
    // function to insert value to end of list
    pub fn insert(&mut self, data: i32) {
        // check if head is nulll
        match self.nodes.len() as i32 {
            // if it is, create new node with the param data
            0 => {
                self.nodes = Vec::new();
                self.nodes.push(*Node::new(data));
                self.head = Some(0);
            }
            // else head has some value
            _ => {
               let mut current: i32 = 0;
               while current< self.nodes.len() as i32 -1{
                current+=1;
               }
               let node = Node::new(data);
               self.nodes.push(*node);
               //print!("curr {}",self.nodes[current as usize].data);
               self.nodes[current as usize].next = Some(current+1);
            }
        }
    }
    //function to delete node
    pub fn delete(&mut self, data: i32){
        // if list is empty, print and return.
        match self.nodes.len() as i32 {
            0 => {
                println!("List is Empty!");
                return;
            }
            // else list is not empty
            _ => {
               
                // set current and prev pointers:
                // Nodes are Option<Box<Node>>, so pointer to node is &mut Option<Box<Node>>
                let mut current: i32 =self.head.unwrap();
                let mut prev: Option<i32> =None;
                // loop through list until value of data is found or we reach the end
                while  current < self.nodes.len() as i32 {
                    // if we find the data:
                    if self.nodes[current as usize].data == data {
                        // check prev pointer to determine course of action
                        match prev {
                            // if prev pointer is still none, our data is in the head variable. we simply need to move head to next variable
                            None => {
                                if self.nodes.len() as i32 > 1{
                                //println!("next , {}",self.nodes[self.head.unwrap() as usize].next.unwrap());
                                self.head = self.nodes[self.head.unwrap() as usize].next
                                }
                                else{
                                    self.head = None;
                                }
                            }
                            // update prev.next to current.next (deletng current)
                            Some(_)=>{                               
                                self.nodes[prev.unwrap() as usize].next = self.nodes[current as usize].next;

                            }
                        }
                        return;
                    }
                    prev = Some(current);
                    current +=1;
                }
                // if we make it through the loop, no value matched,
                return
            }
        }
    }
}
