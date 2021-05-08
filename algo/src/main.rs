mod sort;
// Initializing a Structure
mod studentstruct;
struct Tree {
    value: u16,
    left: *mut Tree,
    right: *mut Tree,
}
impl Tree {
    fn move_left(&self) -> *mut Tree {
        self.left
    }
    fn move_right(&self) -> *mut Tree {
        self.right
    }
}
fn main() {}
