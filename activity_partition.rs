use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::activity::Activity;
use crate::activity_node::ActivityNode;
use crate::element::Element;
use crate::activity_edge::ActivityEdge;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivityPartition {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    in_activity: Option<Weak<RefCell<Activity>>>,
    is_dimension: bool,
    is_external: bool,
    node: Vec<Rc<RefCell<ActivityNode>>>,
    subpartition: Vec<Box<ActivityPartition>>,
    super_partition: Option<Weak<RefCell<ActivityPartition>>>,
    represents: Option<Rc<RefCell<Element>>>,
    edge: Vec<Rc<RefCell<ActivityEdge>>>,
}

impl ActivityPartition {
    pub fn new(is_dimension: bool, is_external: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            in_activity: None,
            is_dimension: is_dimension,
            is_external: is_external,
            node: Vec::new(),
            subpartition: Vec::new(),
            super_partition: None,
            represents: None,
            edge: Vec::new(),
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

    /// Returns a reference to name if present
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Returns a mutable reference to name if present
    pub fn name_mut(&mut self) -> Option<&mut String> {
        self.name.as_mut()
    }

    /// Sets name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Takes name, leaving None in its place
    pub fn take_name(&mut self) -> Option<String> {
        self.name.take()
    }

    /// Returns a reference to visibility if present
    pub fn visibility(&self) -> Option<&String> {
        self.visibility.as_ref()
    }

    /// Returns a mutable reference to visibility if present
    pub fn visibility_mut(&mut self) -> Option<&mut String> {
        self.visibility.as_mut()
    }

    /// Sets visibility
    pub fn set_visibility(&mut self, value: String) {
        self.visibility = Some(value);
    }

    /// Takes visibility, leaving None in its place
    pub fn take_visibility(&mut self) -> Option<String> {
        self.visibility.take()
    }

    /// Returns a reference to client_dependency
    pub fn client_dependency(&self) -> &Vec<Rc<RefCell<Dependency>>> {
        &self.client_dependency
    }

    /// Returns a mutable reference to client_dependency
    pub fn client_dependency_mut(&mut self) -> &mut Vec<Rc<RefCell<Dependency>>> {
        &mut self.client_dependency
    }

    /// Adds an item to client_dependency
    pub fn add_client_dependency(&mut self, item: Rc<RefCell<Dependency>>) {
        self.client_dependency.push(item);
    }

    /// Clears all items from client_dependency
    pub fn clear_client_dependency(&mut self) {
        self.client_dependency.clear();
    }

    /// Returns a reference to name_expression if present
    pub fn name_expression(&self) -> Option<&StringExpression> {
        self.name_expression.as_ref()
    }

    /// Returns a mutable reference to name_expression if present
    pub fn name_expression_mut(&mut self) -> Option<&mut StringExpression> {
        self.name_expression.as_mut()
    }

    /// Sets name_expression
    pub fn set_name_expression(&mut self, value: StringExpression) {
        self.name_expression = Some(value);
    }

    /// Takes name_expression, leaving None in its place
    pub fn take_name_expression(&mut self) -> Option<StringExpression> {
        self.name_expression.take()
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

    /// Returns is_dimension
    pub fn is_dimension(&self) -> bool {
        self.is_dimension
    }

    /// Sets is_dimension
    pub fn set_is_dimension(&mut self, value: bool) {
        self.is_dimension = value;
    }

    /// Returns is_external
    pub fn is_external(&self) -> bool {
        self.is_external
    }

    /// Sets is_external
    pub fn set_is_external(&mut self, value: bool) {
        self.is_external = value;
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

    /// Returns a reference to subpartition
    pub fn subpartition(&self) -> &Vec<Box<ActivityPartition>> {
        &self.subpartition
    }

    /// Returns a mutable reference to subpartition
    pub fn subpartition_mut(&mut self) -> &mut Vec<Box<ActivityPartition>> {
        &mut self.subpartition
    }

    /// Adds an item to subpartition
    pub fn add_subpartition(&mut self, item: ActivityPartition) {
        self.subpartition.push(Box::new(item));
    }

    /// Clears all items from subpartition
    pub fn clear_subpartition(&mut self) {
        self.subpartition.clear();
    }

    /// Returns a reference to super_partition if present
    pub fn super_partition(&self) -> Option<&Weak<RefCell<ActivityPartition>>> {
        self.super_partition.as_ref()
    }

    /// Returns a mutable reference to super_partition if present
    pub fn super_partition_mut(&mut self) -> Option<&mut Weak<RefCell<ActivityPartition>>> {
        self.super_partition.as_mut()
    }

    /// Sets super_partition
    pub fn set_super_partition(&mut self, value: Weak<RefCell<ActivityPartition>>) {
        self.super_partition = Some(value);
    }

    /// Takes super_partition, leaving None in its place
    pub fn take_super_partition(&mut self) -> Option<Weak<RefCell<ActivityPartition>>> {
        self.super_partition.take()
    }

    /// Returns a reference to represents if present
    pub fn represents(&self) -> Option<&Rc<RefCell<Element>>> {
        self.represents.as_ref()
    }

    /// Returns a mutable reference to represents if present
    pub fn represents_mut(&mut self) -> Option<&mut Rc<RefCell<Element>>> {
        self.represents.as_mut()
    }

    /// Sets represents
    pub fn set_represents(&mut self, value: Rc<RefCell<Element>>) {
        self.represents = Some(value);
    }

    /// Takes represents, leaving None in its place
    pub fn take_represents(&mut self) -> Option<Rc<RefCell<Element>>> {
        self.represents.take()
    }

    /// Returns a reference to edge
    pub fn edge(&self) -> &Vec<Rc<RefCell<ActivityEdge>>> {
        &self.edge
    }

    /// Returns a mutable reference to edge
    pub fn edge_mut(&mut self) -> &mut Vec<Rc<RefCell<ActivityEdge>>> {
        &mut self.edge
    }

    /// Adds an item to edge
    pub fn add_edge(&mut self, item: Rc<RefCell<ActivityEdge>>) {
        self.edge.push(item);
    }

    /// Clears all items from edge
    pub fn clear_edge(&mut self) {
        self.edge.clear();
    }

}

impl std::fmt::Display for ActivityPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

