
#[cfg(test)]
mod unit_tests {
    use std::collections::HashMap;

    use crate::parser::{objects::{AddQO, QueryObject, AddNodeQO}, parse_query::parse_query};

    use super::*;
    #[test]
    fn test_add_node(){
        let expected_qo = QueryObject::ADD(AddQO::Node(AddNodeQO {
                identifier: "n1".to_string(),
                type_name: "Person".to_string(),
                properties: HashMap::from([
                    ("name".to_string(), "'Edos'".to_string()), 
                    ("age".to_string(), "20".to_string()),
                ])
            }
        ));
        let query = "ADD NODE n1 TYPE Person PROPERTIES name='Edos', age=20".to_string();
        assert_eq!(parse_query(query).unwrap(), expected_qo);
        let query = "ADD NODE n1 TYPE Person PROPERTIES name='Edos', age=20;".to_string();
        assert_eq!(parse_query(query).unwrap(), expected_qo);
        let query = "ADD NODE n1 TYPE Person PROPERTIES age=20, name='Edos'".to_string();
        assert_eq!(parse_query(query).unwrap(), expected_qo);
    }

    #[test]
    fn test_add_node_fails(){
        let invalid_query = "ADD NODE TYPE".to_string();
        assert!(parse_query(invalid_query).is_err());

        let invalid_query = "ADD NODE n1 TYPE PersonPROPERTIES name='Edos', age=20".to_string();
        assert!(parse_query(invalid_query).is_err());
    }


}
