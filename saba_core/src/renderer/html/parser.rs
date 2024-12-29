use alloc::rc::Rc;
use alloc::rc::Weak;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    window: Weak<RefCell<Window>>,
    parent: Weak<RefCell<Node>>,
    first_child: Option<Rc<RefCell<Node>>>,
    last_child: Option<Weak<RefCell<Node>>>,
    previous_sibling: Option<Weak<RefCell<Node>>>,
    next_sibling: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug, Clone)]
pub struct NodeKind {}

#[derive(Debug, Clone)]
struct Window {}
