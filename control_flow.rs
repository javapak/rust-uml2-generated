use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::activity_node::ActivityNode;
use crate::activity_edge::ActivityEdge;
use crate::activity_partition::ActivityPartition;
use crate::value_specification::ValueSpecification;
use crate::interruptible_activity_region::InterruptibleActivityRegion;
use crate::structured_activity_node::StructuredActivityNode;
use crate::activity::Activity;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlow {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    source: Rc<RefCell<ActivityNode>>,
    target: Rc<RefCell<ActivityNode>>,
    redefined_edge: Vec<Rc<RefCell<ActivityEdge>>>,
    in_partition: Vec<Rc<RefCell<ActivityPartition>>>,
    guard: ValueSpecification,
    weight: ValueSpecification,
    interrupts: Option<Rc<RefCell<InterruptibleActivityRegion>>>,
    in_structured_node: Option<Weak<RefCell<StructuredActivityNode>>>,
    activity: Option<Weak<RefCell<Activity>>>,
}

impl ControlFlow {
    pub fn new(is_leaf: bool, source: Rc<RefCell<ActivityNode>>, target: Rc<RefCell<ActivityNode>>, guard: ValueSpecification, weight: ValueSpecification) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            source: source,
            target: target,
            redefined_edge: Vec::new(),
            in_partition: Vec::new(),
            guard: guard,
            weight: weight,
            interrupts: None,
            in_structured_node: None,
            activity: None,
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

    /// Returns is_leaf
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    /// Sets is_leaf
    pub fn set_is_leaf(&mut self, value: bool) {
        self.is_leaf = value;
    }

    /// Returns a reference to source
    pub fn source(&self) -> &Rc<RefCell<ActivityNode>> {
        &self.source
    }

    /// Returns a mutable reference to source
    pub fn source_mut(&mut self) -> &mut Rc<RefCell<ActivityNode>> {
        &mut self.source
    }

    /// Sets source
    pub fn set_source(&mut self, value: Rc<RefCell<ActivityNode>>) {
        self.source = value;
    }

    /// Returns a reference to target
    pub fn target(&self) -> &Rc<RefCell<ActivityNode>> {
        &self.target
    }

    /// Returns a mutable reference to target
    pub fn target_mut(&mut self) -> &mut Rc<RefCell<ActivityNode>> {
        &mut self.target
    }

    /// Sets target
    pub fn set_target(&mut self, value: Rc<RefCell<ActivityNode>>) {
        self.target = value;
    }

    /// Returns a reference to redefined_edge
    pub fn redefined_edge(&self) -> &Vec<Rc<RefCell<ActivityEdge>>> {
        &self.redefined_edge
    }

    /// Returns a mutable reference to redefined_edge
    pub fn redefined_edge_mut(&mut self) -> &mut Vec<Rc<RefCell<ActivityEdge>>> {
        &mut self.redefined_edge
    }

    /// Adds an item to redefined_edge
    pub fn add_redefined_edge(&mut self, item: Rc<RefCell<ActivityEdge>>) {
        self.redefined_edge.push(item);
    }

    /// Clears all items from redefined_edge
    pub fn clear_redefined_edge(&mut self) {
        self.redefined_edge.clear();
    }

    /// Returns a reference to in_partition
    pub fn in_partition(&self) -> &Vec<Rc<RefCell<ActivityPartition>>> {
        &self.in_partition
    }

    /// Returns a mutable reference to in_partition
    pub fn in_partition_mut(&mut self) -> &mut Vec<Rc<RefCell<ActivityPartition>>> {
        &mut self.in_partition
    }

    /// Adds an item to in_partition
    pub fn add_in_partition(&mut self, item: Rc<RefCell<ActivityPartition>>) {
        self.in_partition.push(item);
    }

    /// Clears all items from in_partition
    pub fn clear_in_partition(&mut self) {
        self.in_partition.clear();
    }

    /// Returns a reference to guard
    pub fn guard(&self) -> &ValueSpecification {
        &self.guard
    }

    /// Returns a mutable reference to guard
    pub fn guard_mut(&mut self) -> &mut ValueSpecification {
        &mut self.guard
    }

    /// Sets guard
    pub fn set_guard(&mut self, value: ValueSpecification) {
        self.guard = value;
    }

    /// Returns a reference to weight
    pub fn weight(&self) -> &ValueSpecification {
        &self.weight
    }

    /// Returns a mutable reference to weight
    pub fn weight_mut(&mut self) -> &mut ValueSpecification {
        &mut self.weight
    }

    /// Sets weight
    pub fn set_weight(&mut self, value: ValueSpecification) {
        self.weight = value;
    }

    /// Returns a reference to interrupts if present
    pub fn interrupts(&self) -> Option<&Rc<RefCell<InterruptibleActivityRegion>>> {
        self.interrupts.as_ref()
    }

    /// Returns a mutable reference to interrupts if present
    pub fn interrupts_mut(&mut self) -> Option<&mut Rc<RefCell<InterruptibleActivityRegion>>> {
        self.interrupts.as_mut()
    }

    /// Sets interrupts
    pub fn set_interrupts(&mut self, value: Rc<RefCell<InterruptibleActivityRegion>>) {
        self.interrupts = Some(value);
    }

    /// Takes interrupts, leaving None in its place
    pub fn take_interrupts(&mut self) -> Option<Rc<RefCell<InterruptibleActivityRegion>>> {
        self.interrupts.take()
    }

    /// Returns a reference to in_structured_node if present
    pub fn in_structured_node(&self) -> Option<&Weak<RefCell<StructuredActivityNode>>> {
        self.in_structured_node.as_ref()
    }

    /// Returns a mutable reference to in_structured_node if present
    pub fn in_structured_node_mut(&mut self) -> Option<&mut Weak<RefCell<StructuredActivityNode>>> {
        self.in_structured_node.as_mut()
    }

    /// Sets in_structured_node
    pub fn set_in_structured_node(&mut self, value: Weak<RefCell<StructuredActivityNode>>) {
        self.in_structured_node = Some(value);
    }

    /// Takes in_structured_node, leaving None in its place
    pub fn take_in_structured_node(&mut self) -> Option<Weak<RefCell<StructuredActivityNode>>> {
        self.in_structured_node.take()
    }

    /// Returns a reference to activity if present
    pub fn activity(&self) -> Option<&Weak<RefCell<Activity>>> {
        self.activity.as_ref()
    }

    /// Returns a mutable reference to activity if present
    pub fn activity_mut(&mut self) -> Option<&mut Weak<RefCell<Activity>>> {
        self.activity.as_mut()
    }

    /// Sets activity
    pub fn set_activity(&mut self, value: Weak<RefCell<Activity>>) {
        self.activity = Some(value);
    }

    /// Takes activity, leaving None in its place
    pub fn take_activity(&mut self) -> Option<Weak<RefCell<Activity>>> {
        self.activity.take()
    }

}

impl Default for ControlFlow {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: false,
            source: Default::default(),
            target: Default::default(),
            redefined_edge: Vec::new(),
            in_partition: Vec::new(),
            guard: Default::default(),
            weight: Default::default(),
            interrupts: None,
            in_structured_node: None,
            activity: None,
        }
    }
}

impl std::fmt::Display for ControlFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

