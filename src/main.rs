// Linked List Driver in Rust
// Michael Rizig
// 9/26/24
mod linkedlist;
use std::{io::{self, Read}, os::macos::raw::stat};
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
                print!("HEAD -> ");
                // set pointer current = head
                let mut current = &list.head;
                // parse throgh list while current!=null, and print value
                while let Some(node) = current {
                    print!("{}", node.data);
                    print!(" -> ");
                    current = &node.next;
                }
                print!("END");
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
                let status = list.delete(intvalue);
                if status{
                    println!("Deleted Succesfully. Current list: ");
                    print!("HEAD -> ");
                    let mut current = &list.head;
                    while let Some(node) = current{
                        print!("{}", node.data);
                        print!(" -> ");
                        current = &node.next;
                    }
                    print!("END");
                }
                input = String::new();
            
            }
            _ => {
                println!("Not matched");
            }
        }
    }
}
