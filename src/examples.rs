/*
*
* Example queries
*
*/

pub const SUBQ_QUERY: &str = "GET NODE SUBQ[UPDATE NODE SET name = 'Malik'] hello FROM SOMEWHERE";
pub const FIND_QUERY: &str = "FIND NODE TYPE Person WHERE condition1 AND condition2 OR condition3";
pub const UPDATE_RELATIONSHIP_QUERY: &str = "UPDATE NODE 1234 SET name = 'Delcos', REMOVE age, ADD age VALUE 21";
pub const UPDATE_NODE_QUERY: &str = "UPDATE NODE 1234 REMOVE height, SET name = 'Delcos', REMOVE age, ADD age VALUE 21";
pub const GET_RELATIONSHIP_QUERY: &str = "GET RELATIONSHIP 7364";
pub const GET_NODE_QUERY: &str = "GET NODE 7364";
pub const REMOVE_RELATIONSHIP_QUERY: &str = "REMOVE RELATIONSHIP 12345";
pub const REMOVE_NODE_QUERY: &str = "REMOVE NODE 12345 MODE CASCADE";
pub const ADD_REL_QUERY: &str = "ADD RELATIONSHIP r1 TYPE LOVES FROM 893641 TO 324218436 PROPERTIES since = 2012, reason = 'natural'";
pub const ADD_NODE_QUERY: &str = "ADD NODE n1 TYPE Person PROPERTIES name = 'Malik', age = 20";
