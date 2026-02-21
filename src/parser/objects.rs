use crate::{
    constants::keywords::{NODE_STR, RELATIONSHIP_STR},
    parser::objects::{
        add::AddQO, find::FindQO, get::GetQO, parse_match::MatchQO, remove::RemoveQO,
        update::UpdateQO,
    },
};

#[derive(Debug, PartialEq)]
pub enum QueryObject {
    Add(AddQO),
    Remove(RemoveQO),
    Get(GetQO),
    Find(FindQO),
    Match(MatchQO),
    Update(UpdateQO),
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

pub mod add {
    use std::collections::HashMap;

    use crate::types::NodeID;

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
}

pub mod remove {
    use crate::{
        constants::keywords::remove::{CASCADE_STR, SAFE_STR},
        types::{NodeID, RelationshipID},
    };

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
}

pub mod get {
    use crate::types::{NodeID, RelationshipID};

    #[derive(Debug, PartialEq)]
    pub enum GetQO {
        Node(NodeID),
        Relationship(RelationshipID),
    }
}

pub mod find {
    #[derive(Debug, PartialEq)]
    pub enum FindQO {
        Node(),
        Nodes(),
        Relationship(),
        Relationships(),
    }
}

pub mod parse_match {
    use std::collections::HashMap;

    use crate::types::IdentifierName;

    #[derive(Debug, PartialEq)]
    pub struct MatchQO {
        pub match_objects: HashMap<IdentifierName, MatchObject>,
        pub filters: HashMap<IdentifierName, FilterCondition>,
        pub return_values: Vec<ReturnValue>,
    }

    #[derive(Debug, PartialEq)]
    pub struct MatchObject {
        pub name: IdentifierName,
        pub object_type: String,
        pub data: IdentifierData,
    }

    #[derive(Debug, PartialEq)]
    pub enum IdentifierData {
        Node {
            outgoing: Option<IdentifierName>,
            ingoing: Option<IdentifierName>,
        },
        Relationship {
            direction: RelationshipDirection,
            start: Option<IdentifierName>,
            end: Option<IdentifierName>,
        },
    }

    #[derive(Debug, PartialEq)]
    pub enum RelationshipDirection {
        Ingoing,
        Outgoing,
    }

    #[derive(Debug, PartialEq)]
    pub struct FilterCondition {}

    #[derive(Debug, PartialEq)]
    pub struct ReturnValue {
        identifier_name: IdentifierName,
        property: Option<String>,
    }

    impl ReturnValue {
        pub fn new(identifier_name: IdentifierName, property: Option<String>) -> Self {
            Self {
                identifier_name,
                property,
            }
        }
    }
}

pub mod update {
    use crate::{
        constants::{
            command_kws::{ADD_STR, REMOVE_STR},
            keywords::update::SET_STR,
        },
        types::NodeID,
    };

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
}

pub struct Subquery {
    query: String,
    priority: u8, // describes the depth in case of recursive subqueries
}
