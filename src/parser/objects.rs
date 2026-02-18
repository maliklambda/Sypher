use crate::constants::{
    command_kws::{ADD_STR, REMOVE_STR},
    keywords::{CASCADE_STR, NODE_STR, RELATIONSHIP_STR, SAFE_STR, SET_STR},
};
use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, PartialEq)]
pub enum QueryObject {
    ADD(AddQO),
    REMOVE(RemoveQO),
    GET(GetQO),
    FIND(FindQO),
    UPDATE(UpdateQO),
}

#[derive(Debug, PartialEq)]
pub enum AddQO {
    Node(AddNodeQO),
    Relationship(AddRelationshipQO),
    Index(),
    Properties(),
    Constraint(),
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
    Node(RemoveNodeQO),
    Relationship(RemoveRelationshipQO),
    Index(),
    Constraint(),
}

#[derive(Debug, PartialEq)]
pub struct RemoveNodeQO {
    pub id: NodeID,
    pub mode: RemoveMode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RemoveMode {
    CASCADE,
    SAFE,
}

impl RemoveMode {
    const STRINGS: &'static [(&'static str, Self)] = &[
        (CASCADE_STR, RemoveMode::CASCADE),
        (SAFE_STR, RemoveMode::SAFE),
    ];

    pub fn from_str(s: &str) -> Option<RemoveMode> {
        let (_, mode) = Self::STRINGS.iter().find(|(value, _)| value == &s)?;
        Some(mode.clone())
    }
}

#[derive(Debug, PartialEq)]
pub struct RemoveRelationshipQO {
    pub id: RelationshipID,
}

#[derive(Debug, PartialEq)]
pub enum GetQO {
    Node(NodeID),
    Relationship(RelationshipID),
}

#[derive(Debug, PartialEq)]
pub enum FindQO {
    Node(),
    Nodes(),
    Relationship(),
    Relationships(),
}

#[derive(Debug, PartialEq)]
pub enum UpdateQO {
    Node(UpdateNodeQO),
    Relationship(UpdateRelationshipQO),
}

#[derive(Debug, PartialEq)]
pub struct UpdateNodeQO {
    pub id: NodeID,
    pub operations: Vec<UpdateOperation>,
}

#[derive(Debug, PartialEq)]
pub enum UpdateOperation {
    Set { property: String, value: String },
    Remove { property: String },
    Add { property: String, value: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateOperationKind {
    SET,
    REMOVE,
    ADD,
}

impl UpdateOperationKind {
    const STRINGS: &'static [(&'static str, UpdateOperationKind)] = &[
        (SET_STR, UpdateOperationKind::SET),
        (REMOVE_STR, UpdateOperationKind::REMOVE),
        (ADD_STR, UpdateOperationKind::ADD),
    ];

    pub fn from_str(s: &str) -> Option<UpdateOperationKind> {
        let (_, operation) = Self::STRINGS.iter().find(|(value, _)| value == &s)?;
        Some(operation.clone())
    }
}

#[derive(Debug, PartialEq)]
pub struct UpdateRelationshipQO {
    pub id: NodeID,
    pub operations: Vec<UpdateOperation>,
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

    pub fn from_str(s: &str) -> Option<ObjectKind> {
        let (_, kind) = Self::STRINGS.iter().find(|(value, _)| value == &s)?;
        Some(kind.clone())
    }
}
