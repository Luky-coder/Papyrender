use crate::style::StyledNode;
use taffy::TaffyTree;

/// Placeholder for the laid-out box tree, backed by taffy.
pub struct LayoutTree(pub TaffyTree<()>);

pub fn compute_layout(_styled: &StyledNode) -> LayoutTree {
    LayoutTree(TaffyTree::new())
}
