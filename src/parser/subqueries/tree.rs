use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::parser::{objects::QueryObject, query::Query};

#[derive(Debug)]
pub struct SubqueryIndexTree {
    pub root: Rc<RefCell<TreeNode>>,
    pub current_nodes: Vec<Rc<RefCell<TreeNode>>>,
    pub indices_map: HashMap<usize, Option<usize>>,
    visited: Vec<TreeNodeValue>,
    queue: VecDeque<Rc<RefCell<TreeNode>>>,
}

impl SubqueryIndexTree {
    pub fn new(value: usize) -> Self {
        let root = TreeNode::new(value);
        SubqueryIndexTree {
            root: root.clone(),
            current_nodes: vec![root.clone()],
            visited: vec![],
            queue: vec![root].into(),
            indices_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: usize) {
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

    pub fn bfs(&self) -> Vec<TreeNodeValue> {
        let mut visited: Vec<TreeNodeValue> = vec![];
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = vec![].into();
        q.push_front(self.root.clone());
        while !q.is_empty() {
            let n = q.pop_back().unwrap();
            visited.push(n.borrow().value.clone());
            for child in n.borrow().children.clone() {
                q.push_front(child);
            }
        }
        visited
    }
}

impl Iterator for SubqueryIndexTree {
    type Item = Rc<RefCell<TreeNode>>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.queue.is_empty() {
            let n = self.queue.pop_back().unwrap();
            self.visited.push(n.borrow().value.clone());
            for child in n.borrow().children.clone() {
                self.queue.push_front(child);
            }
            Some(n)
        } else {
            // reset queue and visited
            self.queue = vec![self.root.clone()].into();
            self.visited = vec![];
            None
        }
    }
}

#[derive(Debug)]
pub struct TreeNode {
    pub value: TreeNodeValue,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(value: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value: TreeNodeValue::NonPrepared(value),
            children: vec![],
        }))
    }
}

pub struct Subquery<'a> {
    pub query: Query<'a>,
    pub children: Vec<Subquery<'a>>,
}

impl<'a> Subquery<'a> {
    pub fn new(query_str: &'a str) -> Self {
        Subquery {
            query: Query::from_str(query_str),
            children: vec![],
        }
    }
}


#[derive(Debug, Clone)]
pub enum TreeNodeValue {
    NonPrepared(usize),
    Prepared {start: usize, end: usize},
    NonParsed (String),
    Parsed (Box<QueryObject>),
}


