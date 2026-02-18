use std::collections::HashMap;
use crate::constants::keywords::{RELATIONSHIP_STR, NODE_STR};

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
    Node (RemoveNodeQO),
    Relationship (RemoveRelationshipQO),
    Index (),
    Constraint (),
}


#[derive(Debug, PartialEq)]
pub struct RemoveNodeQO {
    pub id: NodeID,
    pub mode: RemoveMode
}

#[derive(Debug, PartialEq)]
pub enum RemoveMode {
    CASCADE,
    SAFE,
}

#[derive(Debug, PartialEq)]
pub struct RemoveRelationshipQO {
    pub id: RelationshipID
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



#[derive(Clone, Debug)]
pub enum ObjectKind {
    Node,
    Relationship,
}

impl ObjectKind {
    const STRINGS: &'static [(&'static str, Self)] = &[
        (NODE_STR, ObjectKind::Node),
        (RELATIONSHIP_STR, ObjectKind::Relationship),
    ];

    pub fn from_str (s: &str) -> Option<ObjectKind> {
        let (_, kind) = Self::STRINGS.iter()
            .find(|(value, _)| value == &s)?;
        Some(kind.clone())
    }
}





