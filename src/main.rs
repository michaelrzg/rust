// Linked List Driver in Rust
// Michael Rizig
// 9/26/24
mod linkedlist;
use std::{io::{self, Read}, os::macos::raw::stat};
fn printlist(ll: &linkedlist::LinkedList){
    let mut list = ll;
    print!("HEAD -> ");
     // set pointer current = head
    let mut current: Option<i32> = list.head;
    // parse throgh list while current!=null, and print value
    while current!= None&& current.unwrap()<= list.nodes.len() as i32 {
        print!("{}", list.nodes[current.unwrap() as usize].data);
        print!(" -> ");
        current = list.nodes[current.unwrap() as usize].next;
    }
    print!("END");
   
}
fn main() {
    //string to grab user input
    let mut input = String::new();
    let mut list = linkedlist::LinkedList::new();
    let mut choice = 99;
    while choice != 7 {
        println!("\n\n1 - Insert \n2 - Delete \n3 - Head element \n4 - Size of List \n5 - Is Empty List? \n6 - Print List content \n7 - Exit program \nEnter option number:");
        io::stdin().read_line(&mut input).expect("error reading");

        // trim = remove excess white space
        // parse = gets int value from string
        choice = input.trim().parse().expect("Please enter a valid integer!");
        // switch (choice){}

        match choice {
            // case 1:
            1 => {
                // prompt
                println!("Please enter a value to be inserted into the list: ");
                // variable to store value
                let mut value = String::new();
                // read line
                io::stdin()
                    .read_line(&mut value)
                    .expect("Error reading input");
                // convert to int32
                let intvalue: i32 = value
                    .trim()
                    .parse()
                    .expect("Invalid value. Please enter an integer!");
                // insert converted value into the list
                list.insert(intvalue);

                // print current list values
                println!("Value inserted successfully, current list contents:");
                printlist(&list);
                // reset input string
                input=String::new();
            }
            2=>{
                println!("Please enter a value to be deleted into the list: ");
                // variable to store value
                let mut value = String::new();
                // read in value
                io::stdin().read_line(&mut value).expect("Could not read value!");
                // parse int32 from the string
                let intvalue:i32 = value.trim().parse().expect("Invalid value. Please enter an integer!");
                
                // attempt to delete this value from list:
                list.delete(intvalue);
                printlist(&list);
                input = String::new();
            
            }
            _ => {
                println!("Not matched");
            }
        }
    }
}
