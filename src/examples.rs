#![allow(dead_code)]
/*
*
* Example queries
*
*/

macro_rules! define_constants {
    ($($name:ident = $value:expr),* $(,)?) => {
        $(
            pub const $name: &str = $value;
        )*
        
        pub const EXAMPLE_QUERIES: &[&str] = &[$($name),*];
    };
}

define_constants![
    MATCH_QUERY_INGOING = "MATCH (food:Food) <-[r:LIKES]- (person:Person) WHERE person.name = 'Edos' RETURN person.age, foods.name",
    MATCH_QUERY_OUTGOING = "MATCH (person:Person) -[r:LIKES]-> (food:Food) WHERE person.name = 'Edos' RETURN food.name",
    SUBQ_QUERY = "GET NODE SUBQ{hello}",
    SUBQ_RECURSIVE_QUERY = "MATCH (person:Person) -[r:LIKES]-> (food:Food) WHERE person.name = SUBQ{GET NODE 1234}.name RETURN person.name",
    UPDATE_RELATIONSHIP_QUERY = "UPDATE NODE 1234 SET name = 'Delcos', REMOVE age, ADD age VALUE 21",
    UPDATE_NODE_QUERY = "UPDATE NODE 1234 REMOVE height, SET name = 'Delcos', REMOVE age, ADD age VALUE 21",
    GET_RELATIONSHIP_QUERY = "GET RELATIONSHIP 7364",
    GET_NODE_QUERY = "GET NODE 7364",
    REMOVE_RELATIONSHIP_QUERY = "REMOVE RELATIONSHIP 12345",
    REMOVE_NODE_QUERY = "REMOVE NODE 12345 MODE CASCADE",
    ADD_REL_QUERY = "ADD RELATIONSHIP r1 TYPE LOVES FROM 893641 TO 324218436 PROPERTIES since = 2012, reason = 'natural'",
    ADD_NODE_QUERY = "ADD NODE n1 TYPE Person PROPERTIES name = 'Malik', age = 20",
];


