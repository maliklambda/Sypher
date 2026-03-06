use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    constants::{
        keywords::{
            conditions::{AND_STR, CONDITION_GROUP_END, CONDITION_GROUP_START, OR_STR},
            parse_match::RETURN_STR,
        },
        special_chars::{
            DOUBLE_QUOTE, SINGLE_QUOTE,
            conditions_chars::{EQUAL, GREATER_THAN, SMALLER_THAN},
        },
    },
    parser::{
        errors::{ParseConditionsError, ParseMatchError, ParseConditionsErrorReason},
        objects::parse_match::{ComparisonOperator, Connector, FilterCondition},
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
    pub condition: FilterCondition,
    pub parent: Option<WeakNodePtr>,
    pub and: Option<NodePtr>,
    pub or: Option<NodePtr>,
}

impl Node {
    pub fn new(fc: FilterCondition) -> Self {
        Self {
            condition: fc,
            parent: None,
            and: None,
            or: None,
        }
    }

    pub fn from_condition_vec(
        condition_vec: &Vec<char>,
    ) -> Result<Rc<RefCell<Self>>, ParseMatchError> {
        let condition = parse_single_condition(condition_vec.iter().collect())?;
        Ok(Rc::new(RefCell::new(Self {
            condition,
            parent: None,
            and: None,
            or: None,
        })))
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
    let mut mode = IterMode::Normal;
    let root = Rc::new(RefCell::new(Node::new(FilterCondition::true_condition())));
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
                        let new_node = Node::from_condition_vec(&cond_cur)?;
                        let current_node = s.last().unwrap();
                        Node::add_child(current_node, &new_node, connector_cur);
                        s.push(Rc::clone(&new_node));

                        connector_cur = Connector::And;
                        mode = IterMode::Skip(AND_STR.len() - 1);
                        cond_cur.clear();
                    }
                    'O' if &query.current[idx..idx + OR_STR.len()] == OR_STR => {
                        println!("Finished condition: {:?}", cond_cur);
                        let new_node = Node::from_condition_vec(&cond_cur)?;
                        let current_node = s.last().unwrap();
                        Node::add_child(current_node, &new_node, connector_cur);
                        s.push(Rc::clone(&new_node));

                        connector_cur = Connector::Or;
                        mode = IterMode::Skip(OR_STR.len() - 1);
                        cond_cur.clear();
                    }
                    'R' if &query.current[idx..idx + RETURN_STR.len()] == RETURN_STR => {
                        println!("Finished condition: {:?}", cond_cur);
                        let new_node = Node::from_condition_vec(&cond_cur)?;
                        let current_node = s.last().unwrap();
                        Node::add_child(current_node, &new_node, connector_cur);
                        s.push(Rc::clone(&new_node));

                        mode = IterMode::Ended(idx)
                    }
                    SINGLE_QUOTE => {
                        mode = IterMode::StringSQ;
                        cond_cur.push(c);
                    },
                    DOUBLE_QUOTE => {
                        mode = IterMode::StringDQ;
                        cond_cur.push(c);
                    },
                    CONDITION_GROUP_START => {
                        level += 1;
                        println!("Incrementing condition level to {level}");
                    }
                    CONDITION_GROUP_END => {
                        if level <= 0 {
                            return Err(ParseMatchError::new(
                                crate::parser::errors::ParseMatchErrorReason::ParseConditions { err: ParseConditionsErrorReason::UnclosedGroupEnd },
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
                err: ParseConditionsErrorReason::UnclosedGroupEnd,
            },
            query.current.to_string(),
        ));
    }

    Ok(ConditionTree::new(root))
}

pub fn parse_single_condition(s: String) -> Result<FilterCondition, ParseMatchError> {
    println!("single cond: {s}");
    let (operator, op_idcs) = find_operator(&s)?;
    println!("operator: {:?}", operator);
    let lh = &s[..op_idcs.start].trim();
    println!("Found lh = <{lh}>");
    let rh = &s[op_idcs.start + op_idcs.length..].trim();
    println!("Found rh = <{rh}>");
    Ok(FilterCondition::new(operator, lh.to_string(), rh.to_string()))
}

fn find_operator(s: &str) -> Result<(ComparisonOperator, OperatorIdcs), ParseConditionsError> {
    for (idx, c) in s.chars().enumerate() {
        match c {
            GREATER_THAN => {
                // check for >=
                if s.chars().nth(idx + 1).unwrap() == EQUAL {
                    return Ok((ComparisonOperator::GreaterEqual, OperatorIdcs::new(idx, 2)));
                } else {
                    return Ok((ComparisonOperator::SmallerThan, OperatorIdcs::new(idx, 1)));
                }
            }
            SMALLER_THAN => {
                // check for <=
                if s.chars().nth(idx + 1).unwrap() == EQUAL {
                    return Ok((ComparisonOperator::SmallerEqual, OperatorIdcs::new(idx, 2)));
                } else {
                    return Ok((ComparisonOperator::SmallerThan, OperatorIdcs::new(idx, 1)));
                }
            }
            EQUAL => return Ok((ComparisonOperator::Equal, OperatorIdcs::new(idx, 1))),
            DOUBLE_QUOTE => return Err(ParseConditionsError::new(ParseConditionsErrorReason::LeftHandQuotes, s.to_string())),
            SINGLE_QUOTE => return Err(ParseConditionsError::new(ParseConditionsErrorReason::LeftHandQuotes, s.to_string())),
            _ => {}
        }
    }
    Err(ParseConditionsError::new(ParseConditionsErrorReason::MissingOperator, s.to_string()))
}

pub struct OperatorIdcs {
    start: usize,
    length: usize,
}

impl OperatorIdcs {
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }
}
