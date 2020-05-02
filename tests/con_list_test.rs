enum BList {
    Con(i32, Box<BList>),
    Nil,
}

#[test]
fn basic_blist() {
    use BList::{Con, Nil};
    let a = Con(5, Box::new(Con(6, Box::new(Nil))));
    let _b = Con(3, Box::new(a));
    // let c = Con(3, Box::new(a)); won't compile
}

use std::rc::Rc;

enum RList {
    Con(i16, Rc<RList>),
    Nil,
}

#[test]
fn basic_rlist() {
    use RList::{Con, Nil};
    let a = Rc::new(Con(5, Rc::new(Con(6, Rc::new(Nil)))));
    let _b = Con(3, Rc::clone(&a));
    let _c = Con(3, Rc::clone(&a));
}
