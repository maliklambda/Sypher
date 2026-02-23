// use std::{cell::RefCell, collections::VecDeque, rc::Rc};
//
// use crate::parser::{
//     errors::ParseQueryError, objects::QueryObject, query::Query,
//     subqueries::tree::SubqueryIndexTree,
// };
//
// #[derive(Debug)]
// pub struct QueryTreeNode {
//     query_object: QueryObject,
//     children: Vec<Rc<RefCell<QueryTreeNode>>>,
// }
//
// #[derive(Debug)]
// pub struct QueryTree {
//     root: Rc<RefCell<QueryTreeNode>>,
//     current_nodes: Vec<Rc<RefCell<QueryTreeNode>>>,
//     query_objects: Vec<QueryObject>,
//     queue: VecDeque<Rc<RefCell<QueryTreeNode>>>,
// }
//
// impl QueryTree {
//     pub fn from_raw_tree<'a>(
//         _raw_tree: SubqueryIndexTree,
//         _query: &Query,
//     ) -> Result<Self, ParseQueryError<'a>> {
//         todo!("Query tree from raw query tree");
//     }
//
//     pub fn insert(&mut self, _value: QueryTreeNode) {
//         todo!("insert to query tree");
//         // let new_node = QueryTreeNode::new(value);
//         // println!("{:?}", new_node);
//         // self.current_nodes
//         //     .last_mut()
//         //     .unwrap()
//         //     .borrow_mut()
//         //     .children
//         //     .push(new_node.clone());
//         // self.current_nodes.push(new_node);
//     }
// }
//
// impl Iterator for QueryTree {
//     type Item = Rc<RefCell<QueryTreeNode>>;
//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.queue.is_empty() {
//             let n = self.queue.pop_back().unwrap();
//             self.query_objects.push(n.borrow_mut().query_object.clone());
//             for child in n.borrow().children.clone() {
//                 self.queue.push_front(child);
//             }
//             Some(n)
//         } else {
//             // reset queue and visited
//             self.queue = vec![self.root.clone()].into();
//             self.query_objects = vec![];
//             None
//         }
//     }
// }
