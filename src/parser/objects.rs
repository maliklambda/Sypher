use std::collections::HashMap;

use crate::types::*;


#[derive(Debug)]
pub enum QueryObject {
    ADD (AddQO),
    REMOVE (RemoveQO),
    GET (GetQO),
    FIND (FindQO),
    UPDATE (UpdateQO),
}



#[derive(Debug)]
pub enum AddQO {
    Node (AddNodeQO),
    Relationship (AddRelationshipQO),
    Index (),
    Properties (),
    Constraint (),
}

#[derive(Debug)]
pub struct AddNodeQO {
    pub identifier: String,
    pub type_name: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug)]
pub struct AddRelationshipQO {
    pub identifier: String,
    pub type_name: String,
    pub from: NodeID,
    pub to: NodeID,
    pub properties: HashMap<String, String>,
}


#[derive(Debug)]
pub enum RemoveQO {
    Node (),
    Relationship (),
    Index (),
    Constraint (),
}


#[derive(Debug)]
pub enum GetQO {
    Node (NodeID),
    Relationship (RelationshipID),
}


#[derive(Debug)]
pub enum FindQO {
    Node (),
    Nodes (),
    Relationship (),
    Relationships (),
}



#[derive(Debug)]
pub enum UpdateQO {
    Node (),
    Relationship (),
}



