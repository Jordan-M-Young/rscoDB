use crate::StatementType;

#[derive(Debug)]
pub struct Node {
    node_type: StatementType,
    child_nodes: Vec<Node>,
    mirror_child: bool,
}

pub fn build_select_syntax_tree() -> Node {
    let mut child_nodes: Vec<Node> = vec![];
    let mut select_node = Node {
        node_type: StatementType::SelectType,
        child_nodes,
        mirror_child: false,
    };

    let mut child_nodes: Vec<Node> = vec![];
    let mut columns_node = Node {
        node_type: StatementType::NameType(String::new()),
        child_nodes,
        mirror_child: true,
    };

    let mut child_nodes: Vec<Node> = vec![];
    let mut from_node = Node {
        node_type: StatementType::FromType,
        child_nodes,
        mirror_child: false,
    };

    let mut child_nodes: Vec<Node> = vec![];
    let table_name_node = Node {
        node_type: StatementType::NameType(String::new()),
        child_nodes,
        mirror_child: false,
    };

    from_node.child_nodes.push(table_name_node);
    columns_node.child_nodes.push(from_node);
    select_node.child_nodes.push(columns_node);

    return select_node;
}

pub fn walk_tree(tree: Node) {
    println!("{:?}", tree.node_type);
    for child in tree.child_nodes {
        walk_tree(child)
    }
}
