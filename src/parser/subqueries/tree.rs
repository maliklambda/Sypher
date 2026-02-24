use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::parser::objects::QueryObject;

pub type SubqueryIndex = usize;

#[derive(Debug, Clone)]
pub struct QueryTree {
    pub root: Rc<RefCell<TreeNode>>,
    pub current_nodes: Vec<Rc<RefCell<TreeNode>>>,
    pub indices_map: HashMap<SubqueryIndex, Option<SubqueryPayload>>,
    visited: Vec<SubqueryIndex>,
    queue: VecDeque<Rc<RefCell<TreeNode>>>,
    iter_started: bool, // used to remember state in impl Iterator
}

impl QueryTree {
    pub fn new(value: SubqueryIndex) -> Self {
        let root = TreeNode::new(value);
        QueryTree {
            root: root.clone(),
            current_nodes: vec![root.clone()],
            visited: vec![],
            queue: vec![root].into(),
            indices_map: HashMap::new(),
            iter_started: false,
        }
    }

    pub fn clear_current_nodes(&mut self) {
        self.current_nodes = vec![];
    }

    pub fn clear_queue(&mut self) {
        self.queue = vec![].into();
    }

    pub fn clear_visited(&mut self) {
        self.visited = vec![];
    }

    pub fn insert(&mut self, value: SubqueryIndex) {
        let new_node = TreeNode::new(value);
        println!("{:?}", new_node);
        self.current_nodes
            .last_mut()
            .unwrap()
            .borrow_mut()
            .children
            .push(new_node.clone());
        self.current_nodes.push(new_node);
    }

    pub fn bfs(&self) -> Vec<SubqueryIndex> {
        let mut visited: Vec<SubqueryIndex> = vec![];
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = vec![].into();
        q.push_front(self.root.clone());
        while !q.is_empty() {
            let n = q.pop_back().unwrap();
            visited.push(n.borrow().value);
            for child in n.borrow().children.clone() {
                q.push_front(child);
            }
        }
        visited
    }
}

impl Iterator for QueryTree {
    type Item = Rc<RefCell<TreeNode>>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.iter_started {
            self.queue.push_front(self.root.clone());
            self.iter_started = true;
        }
        if !self.queue.is_empty() {
            let n = self.queue.pop_back().unwrap();
            self.visited.push(n.borrow().value);
            for child in n.borrow().children.clone() {
                self.queue.push_front(child);
            }
            Some(n)
        } else {
            self.clear_queue();
            self.clear_visited();
            self.iter_started = false;
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubqueryPayload {
    pub query_section_end: SubqueryIndex,
    pub query_object: Option<QueryObject>,
}

impl SubqueryPayload {
    pub fn new(value: SubqueryIndex) -> Self {
        SubqueryPayload {
            query_section_end: value,
            query_object: None,
        }
    }
}

#[derive(Debug)]
pub struct TreeNode {
    pub value: SubqueryIndex,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(value: SubqueryIndex) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            children: vec![],
        }))
    }
}
