use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use crate::{
    constants::{
        keywords::{
            conditions::{AND_STR, CONDITION_GROUP_END, CONDITION_GROUP_START, OR_STR},
            parse_match::RETURN_STR,
        },
        special_chars::{DOUBLE_QUOTE, SINGLE_QUOTE},
    },
    parser::{
        errors::ParseMatchError,
        objects::parse_match::{Connector, FilterCondition},
        query::Query,
        subqueries::build_subqueries::IterMode,
    },
};

#[derive(Debug, Clone)]
pub struct ConditionTree {
    root: NodePtr,
}

impl ConditionTree {
    pub fn new(root: NodePtr) -> Self {
        Self { root }
    }
}

impl Iterator for ConditionTree {
    type Item = NodePtr;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("implement iterator for conditiontree");
    }
}

impl PartialEq for ConditionTree {
    fn eq(&self, other: &Self) -> bool {
         std::ptr::eq(self, other)
    }
}

pub type NodePtr = Rc<RefCell<Node>>;
pub type WeakNodePtr = Weak<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    pub condition: String,
    pub parent: Option<WeakNodePtr>,
    pub and: Option<NodePtr>,
    pub or: Option<NodePtr>,
}

impl Node {
    pub fn new(fc: String) -> Self {
        Self {
            condition: fc,
            parent: None,
            and: None,
            or: None,
        }
    }

    pub fn add_child(parent: &NodePtr, child: &NodePtr, connector: Connector) {
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        match connector {
            Connector::And => {
                parent.borrow_mut().and = Some(Rc::clone(child));
            }
            Connector::Or => {
                parent.borrow_mut().or = Some(Rc::clone(child));
            }
            _ => panic!("Cannot add root node to existing node"),
        }
    }
}

pub fn parse_conditions(query: &mut Query) -> Result<ConditionTree, ParseMatchError> {
    let mut filter_conditions: Vec<(Vec<char>, Connector)> = vec![];
    let mut mode = IterMode::Normal;
    let root = Rc::new(RefCell::new(Node::new("ROOT".to_string())));
    let mut s: Vec<NodePtr> = vec![Rc::clone(&root)];
    let mut connector_cur = Connector::And;
    let mut level = 0;
    let mut cond_cur: Vec<char> = vec![];

    for (idx, c) in query.current.chars().enumerate() {
        match mode {
            IterMode::Normal => {
                match c {
                    // AND / OR => parse_single_condition && start new conditions_str
                    'A' if &query.current[idx..idx + AND_STR.len()] == AND_STR => {
                        println!("Finished condition: {:?}", cond_cur);
                        let new_node = Rc::new(RefCell::new(Node::new(cond_cur.iter().collect())));
                        let current_node = s.last().unwrap();
                        Node::add_child(current_node, &new_node, connector_cur);
                        s.push(Rc::clone(&new_node));

                        filter_conditions.push((cond_cur.clone(), connector_cur));
                        connector_cur = Connector::And;
                        mode = IterMode::Skip(AND_STR.len() - 1);
                        cond_cur.clear();
                    }
                    'O' if &query.current[idx..idx + OR_STR.len()] == OR_STR => {
                        println!("Finished condition: {:?}", cond_cur);
                        let new_node = Rc::new(RefCell::new(Node::new(cond_cur.iter().collect())));
                        let current_node = s.last().unwrap();
                        Node::add_child(current_node, &new_node, connector_cur);
                        s.push(Rc::clone(&new_node));

                        filter_conditions.push((cond_cur.clone(), connector_cur));
                        connector_cur = Connector::Or;
                        mode = IterMode::Skip(OR_STR.len() - 1);
                        cond_cur.clear();
                    }
                    'R' if &query.current[idx..idx + RETURN_STR.len()] == RETURN_STR => {
                        println!("Finished condition: {:?}", cond_cur);
                        let new_node = Rc::new(RefCell::new(Node::new(cond_cur.iter().collect())));
                        let current_node = s.last().unwrap();
                        Node::add_child(current_node, &new_node, connector_cur);
                        s.push(Rc::clone(&new_node));

                        filter_conditions.push((cond_cur.clone(), connector_cur));
                        mode = IterMode::Ended(idx)
                    }
                    SINGLE_QUOTE => mode = IterMode::StringSQ,
                    DOUBLE_QUOTE => mode = IterMode::StringDQ,
                    CONDITION_GROUP_START => {
                        level += 1;
                        println!("Incrementing condition level to {level}");
                    }
                    CONDITION_GROUP_END => {
                        if level <= 0 {
                            return Err(ParseMatchError::new(
                                crate::parser::errors::ParseMatchErrorReason::ParseConditions {err: crate::parser::errors::ParseConditionsError::UnclosedGroupStart},
                                query.current.to_string(),
                            ));
                        }
                        s.pop();
                        level -= 1;
                        println!("Decrementing condition level to {level}");
                    }
                    _ => cond_cur.push(c),
                }
            }
            IterMode::StringDQ => {
                if c == DOUBLE_QUOTE {
                    mode = IterMode::Normal;
                }
                cond_cur.push(c)
            }
            IterMode::StringSQ => {
                if c == SINGLE_QUOTE {
                    mode = IterMode::Normal;
                }
                cond_cur.push(c)
            }
            IterMode::Skip(num) => {
                mode = if num > 1 {
                    IterMode::Skip(num - 1)
                } else {
                    IterMode::Normal
                }
            }
            IterMode::Ended(idx) => break,
            _ => {}
        }
        println!("char: {}", c);
    }

    if let IterMode::Ended(n) = mode {
        query.trim_n_left(n);
    } else {
        panic!(
            "Expected iterMode to end with status 'Ended', got: {:?}",
            mode
        );
    }

    if level > 0 {
        return Err(ParseMatchError::new(
            crate::parser::errors::ParseMatchErrorReason::ParseConditions {
                err: crate::parser::errors::ParseConditionsError::UnclosedGroupEnd,
            },
            query.current.to_string(),
        ));
    }

    println!("conditions: {:?}", filter_conditions);
    Ok(ConditionTree::new(root))
}

pub fn parse_single_condition(v: Vec<char>) -> Result<FilterCondition, ParseMatchError> {
    todo!("parse single condition");
}
