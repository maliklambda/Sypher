use std::collections::HashMap;

use crate::types::*;


#[derive(Debug, PartialEq)]
pub enum QueryObject {
    ADD (AddQO),
    REMOVE (RemoveQO),
    GET (GetQO),
    FIND (FindQO),
    UPDATE (UpdateQO),
}



#[derive(Debug, PartialEq)]
pub enum AddQO {
    Node (AddNodeQO),
    Relationship (AddRelationshipQO),
    Index (),
    Properties (),
    Constraint (),
}

#[derive(Debug, PartialEq)]
pub struct AddNodeQO {
    pub identifier: String,
    pub type_name: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub struct AddRelationshipQO {
    pub identifier: String,
    pub type_name: String,
    pub from: NodeID,
    pub to: NodeID,
    pub properties: HashMap<String, String>,
}


#[derive(Debug, PartialEq)]
pub enum RemoveQO {
    Node (),
    Relationship (),
    Index (),
    Constraint (),
}


#[derive(Debug, PartialEq)]
pub enum GetQO {
    Node (NodeID),
    Relationship (RelationshipID),
}


#[derive(Debug, PartialEq)]
pub enum FindQO {
    Node (),
    Nodes (),
    Relationship (),
    Relationships (),
}



#[derive(Debug, PartialEq)]
pub enum UpdateQO {
    Node (),
    Relationship (),
}



#[derive(Debug)]
pub struct NodeTuple {
    pub from: u32,
    pub to: u32,
}

impl NodeTuple {
    pub fn new(to: u32, from: u32) -> NodeTuple {
        NodeTuple { from, to }
    }
}
