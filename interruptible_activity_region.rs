use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::activity::Activity;
use crate::activity_node::ActivityNode;
use crate::activity_edge::ActivityEdge;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterruptibleActivityRegion {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    in_activity: Option<Weak<RefCell<Activity>>>,
    node: Vec<Rc<RefCell<ActivityNode>>>,
    interrupting_edge: Vec<Rc<RefCell<ActivityEdge>>>,
}

impl InterruptibleActivityRegion {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            in_activity: None,
            node: Vec::new(),
            interrupting_edge: Vec::new(),
        }
    }

    /// Returns a slice of e_annotations
    pub fn e_annotations(&self) -> &[EAnnotation] {
        &self.e_annotations
    }

    /// Returns a mutable reference to e_annotations
    pub fn e_annotations_mut(&mut self) -> &mut Vec<EAnnotation> {
        &mut self.e_annotations
    }

    /// Adds an item to e_annotations
    pub fn add_e_annotation(&mut self, item: EAnnotation) {
        self.e_annotations.push(item);
    }

    /// Clears all items from e_annotations
    pub fn clear_e_annotations(&mut self) {
        self.e_annotations.clear();
    }

    /// Returns a slice of owned_comment
    pub fn owned_comment(&self) -> &[Comment] {
        &self.owned_comment
    }

    /// Returns a mutable reference to owned_comment
    pub fn owned_comment_mut(&mut self) -> &mut Vec<Comment> {
        &mut self.owned_comment
    }

    /// Adds an item to owned_comment
    pub fn add_owned_comment(&mut self, item: Comment) {
        self.owned_comment.push(item);
    }

    /// Clears all items from owned_comment
    pub fn clear_owned_comment(&mut self) {
        self.owned_comment.clear();
    }

    /// Returns a reference to in_activity if present
    pub fn in_activity(&self) -> Option<&Weak<RefCell<Activity>>> {
        self.in_activity.as_ref()
    }

    /// Returns a mutable reference to in_activity if present
    pub fn in_activity_mut(&mut self) -> Option<&mut Weak<RefCell<Activity>>> {
        self.in_activity.as_mut()
    }

    /// Sets in_activity
    pub fn set_in_activity(&mut self, value: Weak<RefCell<Activity>>) {
        self.in_activity = Some(value);
    }

    /// Takes in_activity, leaving None in its place
    pub fn take_in_activity(&mut self) -> Option<Weak<RefCell<Activity>>> {
        self.in_activity.take()
    }

    /// Returns a reference to node
    pub fn node(&self) -> &Vec<Rc<RefCell<ActivityNode>>> {
        &self.node
    }

    /// Returns a mutable reference to node
    pub fn node_mut(&mut self) -> &mut Vec<Rc<RefCell<ActivityNode>>> {
        &mut self.node
    }

    /// Adds an item to node
    pub fn add_node(&mut self, item: Rc<RefCell<ActivityNode>>) {
        self.node.push(item);
    }

    /// Clears all items from node
    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    /// Returns a reference to interrupting_edge
    pub fn interrupting_edge(&self) -> &Vec<Rc<RefCell<ActivityEdge>>> {
        &self.interrupting_edge
    }

    /// Returns a mutable reference to interrupting_edge
    pub fn interrupting_edge_mut(&mut self) -> &mut Vec<Rc<RefCell<ActivityEdge>>> {
        &mut self.interrupting_edge
    }

    /// Adds an item to interrupting_edge
    pub fn add_interrupting_edge(&mut self, item: Rc<RefCell<ActivityEdge>>) {
        self.interrupting_edge.push(item);
    }

    /// Clears all items from interrupting_edge
    pub fn clear_interrupting_edge(&mut self) {
        self.interrupting_edge.clear();
    }

}

impl std::fmt::Display for InterruptibleActivityRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterruptibleActivityRegion(...)")
    }
}

