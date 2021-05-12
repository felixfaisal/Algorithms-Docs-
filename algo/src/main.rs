mod linkedlist;
struct myList {
    value: u32,
    next: address,
}
enum address {
    address(Box<myList>),
    Nil,
}
impl myList {
    fn new(elem: u32) -> myList {
        myList {
            value: elem,
            next: address::Nil,
        }
    }
    fn append(mut self, elem: u32) -> myList {
        let node = myList {
            value: elem,
            next: address::Nil,
        };
        self.next = address::address(Box::new(node));
        self
    }
    fn append2(&mut self, elem: u32) {
        match self.next {
            address::address(ref mut nextaddress) => {
                nextaddress.append2(elem);
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
    fn list(&self) {
        println!("{}", self.value);
        match self.next {
            address::address(ref nextaddress) => {
                nextaddress.list();
            }
            address::Nil => {
                println!("Nil")
            }
        }
    }
}
fn main() {
    let mut head = myList {
        value: 8,
        next: address::Nil,
    };
    head.append2(15);
    head.append2(19);
    head.list();

    match head.next {
        address::address(ref addressX) => {
            println!("This has a next address containing, {}", addressX.value);
        }
        address::Nil => {}
    }
}
