use std::{
    cell::RefCell,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};

struct StackNode<T: Clone + Debug> {
    value: T,
    next: Rc<RefCell<Option<StackNode<T>>>>,
}

impl<T: Clone + Debug> Debug for StackNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("StackNode")
            .field("value", &self.value)
            .field("next", &self.next)
            .finish()
    }
}

struct Stack<T: Clone + Debug> {
    head: Rc<RefCell<Option<StackNode<T>>>>,
}

impl<T: Clone + Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Stack").field("head", &self.head).finish()
    }
}

pub fn create_rc<T>(item: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(item))
}

impl<T: Clone + Debug> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: create_rc(None),
        }
    }

    pub fn push(&mut self, item: T) -> () {
        self.head = create_rc(Some(StackNode {
            value: item.clone(),
            next: Rc::clone(&self.head),
        }));

        ()
    }

    pub fn pop(&mut self) -> () {
        let head = Rc::clone(&self.head);

        match &*head.borrow() {
            Some(x) => {
                let next = Rc::clone(&x.next);

                if next.borrow().is_none() {
                    self.head = create_rc(None);
                }

                self.head = next;
            }
            None => (),
        }

        ()
    }

    pub fn peek(&mut self) -> Option<T> {
        let head = Rc::clone(&self.head);

        let x = head.borrow();

        match &*x {
            Some(node) => Some(node.value.clone()),
            None => None,
        }
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.pop();

    println!("{:?}", stack);
}
