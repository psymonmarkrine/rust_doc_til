use std::rc::Rc;
use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {
    let _list = Cons(1, 
        Rc::new(Cons(2,
            Rc::new(Cons(3,
                Rc::new(Nil))))));

    let a = Rc::new(Cons(5,
                Rc::new(Cons(10,
                    Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
