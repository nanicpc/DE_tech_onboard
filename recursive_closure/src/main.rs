# based on code from lwouters@cenotelie.fr
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let closure: Rc<RefCell<Option<Box<dyn Fn(Vec<usize>,Vec<(usize,usize)>) -> ()>>>> = Rc::new(RefCell::new(None));
    //                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Here my function has two arguments of type Vector 
    //                             ^^^^^^^^^^^^^^^^^^^^^^^^ the closure, boxed because we need a concrete type now
    //                      ^^^^^^ option of a closure because we need the value before the closure is constructed, so we we start with None
    //               ^^^^^^^ RefCell because we need to modify the option after it is shared to set the closure
    //            ^^ Rc to share the closure and have a reference to itself embedded inside the closure itself
    let closure_inner = closure.clone();
    let increment = 2;
    //  ^^^^^^^^^^^^^ this will be embedded inside the closure itself
    *closure.borrow_mut() = Some(Box::new(move |mut n, mut t| {
    //                                    ^^^^ we move closure_inner inside the closure ...
        println!("{:?}", n);
        n.push(n[n.len()-1]+increment);
        t.push((n[0],n[n.len()-1]));
        println!("{:?}", t);
        if n.len() < 10 {
            closure_inner.borrow().as_ref().unwrap()(n,t);
            // ^^^^ ... so that we can use it here
        }
        
    }));

    // the call is a bit convoluted ...
    closure.borrow().as_ref().unwrap()(vec![0],vec![]);
    //      ^^^^^^ on RefCell, immutably borrow a smart pointer on the Option
    //               ^^^^^^^ on the Ref<Option<Box<dyn Fn(arguments) -> ()>>>, we get an Option<&Box<dyn Fn(arguments) -> ()>>> (so as not not consume)
    //                        ^^^^^^^ we know that the value is initialized, get rid of the Option so that in the end we have
    //  just a &Box<dyn Fn(arguments) -> ()>>, that behaves as a &dyn Fn(arguments) -> (), i.e. a pointer to a something that can be called
    //  ... and is call as a function :)
}