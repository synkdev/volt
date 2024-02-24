use taffy::prelude::*;

fn layout() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let header_node = tree
        .new_leaf(Style {
            size: Size {
                width: length(800.0),
                height: length(100.0),
            },
            ..Default::default()
        })
        .unwrap();

    let body_node = tree
        .new_leaf(Style {
            size: Size {
                width: length(800.0),
                height: auto(),
            },
            flex_grow: 1.0,
            ..Default::default()
        })
        .unwrap();

    let root_node = tree
        .new_with_children(
            Style {
                flex_direction: FlexDirection::Column,
                size: Size {
                    width: length(800.0),
                    height: length(600.0),
                },
                ..Default::default()
            },
            &[header_node, body_node],
        )
        .unwrap();

    tree.compute_layout(root_node, Size::MAX_CONTENT).unwrap();
}
