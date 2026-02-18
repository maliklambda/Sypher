use crate::parser::objects::{QueryObject, RemoveMode::CASCADE, RemoveNodeQO, RemoveQO};

#[test]
fn test_remove_node() {
    let expected_qo = QueryObject::REMOVE(RemoveQO::Node(RemoveNodeQO {
        id: 1234,
        mode: crate::parser::objects::RemoveMode::CASCADE,
    }));

}

fn test_remove_node_fails() {}

fn test_remove_relationship() {}

fn test_remove_relationship_fails() {}
