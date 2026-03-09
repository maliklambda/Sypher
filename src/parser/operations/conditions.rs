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
        errors::{ParseConditionsError, ParseConditionsErrorReason, ParseMatchError},
        objects::parse_match::{ComparisonOperator, Connector, FilterCondition},
        operations::expressions::parse_expression,
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

    pub fn iter(&self) -> ConditionTreeIterator {
        let mut nodes = inorder_traverse(self.root.clone());
        nodes.pop(); // pop root element with "always true" condition
        ConditionTreeIterator::new(nodes)
    }
}

pub struct ConditionTreeIterator {
    nodes: Vec<Node>,
    cur_idx: usize,
}

impl ConditionTreeIterator {
    fn new(nodes: Vec<Node>) -> Self {
        Self { nodes, cur_idx: 0 }
    }
}

impl Iterator for ConditionTreeIterator {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.nodes.get(self.cur_idx).cloned();
        self.cur_idx += 1;
        val
    }
}

fn inorder_traverse(tree: NodePtr) -> Vec<Node> {
    let mut result: Vec<Node> = Vec::new();
    inorder(Some(tree), &mut result);
    result
}

fn inorder(tree: Option<NodePtr>, result: &mut Vec<Node>) -> Option<()> {
    if tree.is_none() {
        return None;
    } // Return None if we reach a None value

    let current_tree = tree.unwrap();
    let current_value = current_tree.borrow().val.clone();

    inorder(current_tree.to_owned().borrow().and.to_owned(), result);
    result.push(current_value);
    inorder(current_tree.to_owned().borrow().or.to_owned(), result);

    Some(())
}

impl Iterator for ConditionTree {
    type Item = NodePtr;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("iterator for condtion tree")
    }
}

impl PartialEq for ConditionTree {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

pub type NodePtr = Rc<RefCell<NodeWrapper>>;
pub type WeakNodePtr = Weak<RefCell<NodeWrapper>>;

#[derive(Debug)]
pub struct NodeWrapper {
    parent: Option<WeakNodePtr>,
    pub and: Option<NodePtr>,
    pub or: Option<NodePtr>,
    pub val: Node,
}

impl NodeWrapper {
    pub fn from_atom(atom: AtomNode) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent: None,
            and: None,
            or: None,
            val: Node::Atom(atom),
        }))
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

    pub fn has_and(&self) -> bool {
        self.and.is_some()
    }

    pub fn has_or(&self) -> bool {
        self.or.is_some()
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Atom(AtomNode),
    Tree(ConditionTree),
}

#[derive(Debug, Clone)]
pub struct AtomNode {
    pub condition: FilterCondition,
}

impl std::fmt::Display for AtomNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.condition)
    }
}

impl AtomNode {
    pub fn new(fc: FilterCondition) -> Self {
        Self { condition: fc }
    }

    pub fn from_condition_vec(condition_vec: &Vec<char>) -> Result<Self, ParseMatchError> {
        let condition = parse_single_condition(condition_vec.iter().collect())?;
        Ok(Self { condition })
    }
}

pub fn parse_conditions(query: &mut Query) -> Result<ConditionTree, ParseMatchError> {
    let mut mode = IterMode::Normal;

    // root is a dummy node whose condition is always true
    // This saves repetitive checks in iteration below
    let root = NodeWrapper::from_atom(AtomNode::new(FilterCondition::true_condition()));
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
                        let new_node =
                            NodeWrapper::from_atom(AtomNode::from_condition_vec(&cond_cur)?);
                        let current_node = s.last().unwrap();
                        NodeWrapper::add_child(current_node, &new_node, connector_cur);
                        s.push(Rc::clone(&new_node));

                        connector_cur = Connector::And;
                        mode = IterMode::Skip(AND_STR.len() - 1);
                        cond_cur.clear();
                    }
                    'O' if &query.current[idx..idx + OR_STR.len()] == OR_STR => {
                        println!("Finished condition: {:?}", cond_cur);
                        let new_node =
                            NodeWrapper::from_atom(AtomNode::from_condition_vec(&cond_cur)?);
                        let current_node = s.last().unwrap();
                        NodeWrapper::add_child(current_node, &new_node, connector_cur);
                        s.push(Rc::clone(&new_node));

                        connector_cur = Connector::Or;
                        mode = IterMode::Skip(OR_STR.len() - 1);
                        cond_cur.clear();
                    }
                    'R' if &query.current[idx..idx + RETURN_STR.len()] == RETURN_STR => {
                        println!("Finished condition: {:?}", cond_cur);
                        let new_node =
                            NodeWrapper::from_atom(AtomNode::from_condition_vec(&cond_cur)?);
                        let current_node = s.last().unwrap();
                        NodeWrapper::add_child(current_node, &new_node, connector_cur);
                        s.push(Rc::clone(&new_node));

                        mode = IterMode::Ended(idx)
                    }
                    SINGLE_QUOTE => {
                        mode = IterMode::StringSQ;
                        cond_cur.push(c);
                    }
                    DOUBLE_QUOTE => {
                        mode = IterMode::StringDQ;
                        cond_cur.push(c);
                    }
                    CONDITION_GROUP_START => {
                        todo!("handle grouped conditions, e.g. 'WHERE A AND (B OR (C AND D) AND E) AND F'");
                        level += 1;
                        println!("Incrementing condition level to {level}");
                    }
                    CONDITION_GROUP_END => {
                        if level <= 0 {
                            println!("Unclode GE @ {idx}");
                            return Err(ParseMatchError::new(
                                crate::parser::errors::ParseMatchErrorReason::ParseConditions {
                                    err: ParseConditionsErrorReason::UnclosedGroupEnd,
                                },
                                query.current.to_string(),
                            ));
                        }
                        level -= 1;
                        println!("Found s = {:?}", s);
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

    let n = {
        if let IterMode::Ended(n) = mode {
            query.trim_n_left(n);
            n
        } else {
            panic!(
                "Expected iterMode to end with status 'Ended', got: {:?}",
                mode
            );
        }
    };

    if level > 0 {
        println!("Unclode GE @ {n}");
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
    let lh_expr = parse_expression(lh)?;
    println!("Found lh = <{:?}>", lh_expr);

    let rh = &s[op_idcs.start + op_idcs.length..].trim();
    let rh_expr = parse_expression(rh)?;
    println!("Found rh = <{:?}>", rh_expr);
    Ok(FilterCondition::new(operator, lh_expr, rh_expr))
}

fn find_operator(s: &str) -> Result<(ComparisonOperator, OperatorIdcs), ParseConditionsError> {
    for (idx, c) in s.chars().enumerate() {
        match c {
            GREATER_THAN => {
                // check for >=
                if s.chars().nth(idx + 1).unwrap() == EQUAL {
                    return Ok((ComparisonOperator::GreaterEqual, OperatorIdcs::new(idx, 2)));
                } else {
                    return Ok((ComparisonOperator::GreaterThan, OperatorIdcs::new(idx, 1)));
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
            DOUBLE_QUOTE => {
                return Err(ParseConditionsError::new(
                    ParseConditionsErrorReason::LeftHandQuotes,
                    s.to_string(),
                ));
            }
            SINGLE_QUOTE => {
                return Err(ParseConditionsError::new(
                    ParseConditionsErrorReason::LeftHandQuotes,
                    s.to_string(),
                ));
            }
            _ => {}
        }
    }
    Err(ParseConditionsError::new(
        ParseConditionsErrorReason::MissingOperator,
        s.to_string(),
    ))
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
