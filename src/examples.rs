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
    MATCH_CONDITIONS_COMPLEX = "MATCH (p:) <- (f:Food) WHERE p.name = 'Edos' AND p.age > 23 AND p.balance*2 >= 500.00 OR p.debt%2 -100 < -34.0 AND a=4 RETURN p.name, f.cuisine",
    MATCH_CONDITIONS_GROUPED =
        "MATCH (p:) <- (f:Food) WHERE (p.name = 'name' AND p.age = 23) RETURN p.name, f.cuisine",
    MATCH_CONDITIONS = "MATCH (p:) <- (f:Food) WHERE p.name = 'Edos' AND p.age > 23 AND p.balance >= 500.00 OR p.debt < -34.0 AND a=4 RETURN p.name, f.cuisine",
    MATCH_SIMPLE_INGOING =
        "MATCH (p:) <- (f:Food) WHERE p.name = 'some_name' RETURN p.name, f.cuisine",
    MATCH_SIMPLE_OUTGOING =
        "MATCH (p:Person) -> (f:Food) WHERE p.name = 'some_name' RETURN p.name, f.cuisine",
    MATCH_INGOING = "MATCH (food:Food) <-[r:LIKES]- (person:Person) WHERE person.name = 'Edos' RETURN person.age, foods.name",
    MATCH_OUTGOING = "MATCH (person:Person) -[r:LIKES]-> (food:Food) WHERE person.name = 'Edos' RETURN food.name",
    SUBQ = "GET NODE SUBQ{GET RELATIONSHIP 234}",
    SUBQ_RECURSIVE = "MATCH (person:Person) -[r:LIKES]-> (food:Food) WHERE person.name = SUBQ{GET NODE 1234}.name RETURN person.name",
    UPDATE_RELATIONSHIP = "UPDATE NODE 1234 SET name = 'Delcos', REMOVE age, ADD age VALUE 21",
    UPDATE_NODE =
        "UPDATE NODE 1234 REMOVE height, SET name = 'Delcos', REMOVE age, ADD age VALUE 21",
    GET_RELATIONSHIP = "GET RELATIONSHIP 7364",
    GET_NODE = "GET NODE 7364",
    REMOVE_RELATIONSHIP = "REMOVE RELATIONSHIP 12345",
    REMOVE_NODE = "REMOVE NODE 12345 MODE CASCADE",
    ADD_REL = "ADD RELATIONSHIP r1 TYPE LOVES FROM 893641 TO 324218436 PROPERTIES since = 2012, reason = 'natural'",
    ADD_NODE = "ADD NODE n1 TYPE Person PROPERTIES name = 'Malik', age = 20",
];
