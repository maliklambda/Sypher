use crate::{
    constants::keywords::{NODE_STR, RELATIONSHIP_STR},
    parser::objects::{
        add::AddQO, get::GetQO, parse_match::MatchQO, remove::RemoveQO, update::UpdateQO,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub enum QueryObject {
    Add(AddQO),
    Remove(RemoveQO),
    Get(GetQO),
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

    #[derive(Debug, PartialEq, Clone)]
    pub enum AddQO {
        Node(AddNodeQO),
        Relationship(AddRelationshipQO),
        Index(),
        Properties(),
        Constraint(),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct AddNodeQO {
        pub identifier: String,
        pub type_name: String,
        pub properties: HashMap<String, String>,
    }

    #[derive(Debug, PartialEq, Clone)]
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

    #[derive(Debug, PartialEq, Clone)]
    pub enum RemoveQO {
        Node(RemoveNodeQO),
        Relationship(RemoveRelationshipQO),
        Index(),
        Constraint(),
    }

    #[derive(Debug, PartialEq, Clone)]
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

    #[derive(Debug, PartialEq, Clone)]
    pub struct RemoveRelationshipQO {
        pub id: RelationshipID,
    }
}

pub mod get {
    use crate::types::{NodeID, RelationshipID};

    #[derive(Debug, PartialEq, Clone)]
    pub enum GetQO {
        Node(NodeID),
        Relationship(RelationshipID),
    }
}

pub mod parse_match {
    use std::collections::HashMap;

    use crate::{
        parser::operations::{
            conditions::ConditionTree,
            expressions::{ConstantExpression, Expression, SimpleExpression},
        },
        types::IdentifierName,
    };

    #[derive(Debug, PartialEq, Clone)]
    pub struct MatchQO {
        pub match_objects: HashMap<IdentifierName, MatchObject>,
        pub condition_tree: ConditionTree,
        pub return_values: Vec<ReturnValue>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct MatchObject {
        pub name: IdentifierName,
        pub object_type: String,
        pub data: IdentifierData,
    }

    impl MatchObject {
        pub fn new(
            name: IdentifierName,
            type_name: String,
            data: IdentifierData,
        ) -> Self {
            MatchObject {
                name,
                object_type: type_name,
                data,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
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

    #[derive(Debug, PartialEq, Clone)]
    pub enum RelationshipDirection {
        Ingoing,
        Outgoing,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct FilterCondition {
        left_side: Expression,
        comparison_operator: ComparisonOperator,
        right_side: Expression,
    }

    impl FilterCondition {
        pub fn new(cmp: ComparisonOperator, lh: Expression, rh: Expression) -> Self {
            Self {
                comparison_operator: cmp,
                left_side: lh,
                right_side: rh,
            }
        }
        pub fn true_condition() -> FilterCondition {
            Self {
                left_side: Expression::Simple(SimpleExpression::Constant(
                    ConstantExpression::Int32(0),
                )),
                comparison_operator: ComparisonOperator::Equal,
                right_side: Expression::Simple(SimpleExpression::Constant(
                    ConstantExpression::Int32(0),
                )),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Connector {
        And,
        Or,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum ComparisonOperator {
        Equal,
        GreaterThan,
        GreaterEqual,
        SmallerThan,
        SmallerEqual,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReturnValue {
        pub identifier_name: IdentifierName,
        pub property: Option<String>,
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

    #[derive(Debug, PartialEq, Clone)]
    pub enum UpdateQO {
        Node(UpdateNodeQO),
        Relationship(UpdateRelationshipQO),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct UpdateNodeQO {
        pub id: NodeID,
        pub operations: Vec<UpdateOperation>,
    }

    #[derive(Debug, PartialEq, Clone)]
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

    #[derive(Debug, PartialEq, Clone)]
    pub struct UpdateRelationshipQO {
        pub id: NodeID,
        pub operations: Vec<UpdateOperation>,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Subquery<'a> {
    pub query: &'a str,
}

impl<'a> Subquery<'a> {
    pub fn new(s: &'a str) -> Self {
        Subquery { query: s }
    }
}

impl<'a> std::fmt::Display for Subquery<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SUBQUERY: '{}'", self.query)
    }
}
