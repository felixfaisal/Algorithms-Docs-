mod linkedlist;
#[derive(Clone)]
struct myList {
    value: u32,
    next: address,
}
#[derive(Clone)]
enum address {
    address(Box<myList>),
    Nil,
}
impl myList {
    fn append(&mut self, elem: u32) {
        match self.next {
            address::address(ref mut next_address) => {
                next_address.append(elem);
            }
            address::Nil => {
                let node = myList {
                    value: elem,
                    next: address::Nil,
                };
                self.next = address::address(Box::new(node))
            }
        }
    }
    fn delete(&mut self, elem: u32) {
        match self.next {
            address::address(ref mut next_address) => {
                if next_address.value == elem {
                    println!("Deleting value {}", next_address.value);
                    self.next = next_address.next.clone();
                } else {
                    next_address.delete(elem);
                }
            }
            address::Nil => {
                if self.value == elem {
                    self.value = 0;
                } else {
                    println!("Element {} does not exist in the linked list", elem);
                }
            }
        }
    }
    fn count(&self) -> u32 {
        match self.next {
            address::address(ref newaddress) => 1 + newaddress.count(),
            address::Nil => 0,
        }
    }
    fn list(&self) {
        if self.value == 0 {
            println!("The list is empty")
        } else {
            println!("{}", self.value);
            match self.next {
                address::address(ref next_address) => next_address.list(),
                address::Nil => {}
            }
        }
    }
}
fn main() {
    let mut head = myList {
        value: 8,
        next: address::Nil,
    };
    head.append(9);
    head.append(10);
    head.append(11);
    head.list();
    println!("The size of the list is {}", head.count());
}
