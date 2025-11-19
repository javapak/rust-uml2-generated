use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::structured_activity_node::StructuredActivityNode;
use crate::activity::Activity;
use crate::activity_edge::ActivityEdge;
use crate::activity_partition::ActivityPartition;
use crate::interruptible_activity_region::InterruptibleActivityRegion;
use crate::activity_node::ActivityNode;
use crate::exception_handler::ExceptionHandler;
use crate::constraint::Constraint;
use crate::variable::Variable;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearVariableAction {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    in_structured_node: Option<Weak<RefCell<StructuredActivityNode>>>,
    activity: Option<Weak<RefCell<Activity>>>,
    outgoing: Vec<Rc<RefCell<ActivityEdge>>>,
    incoming: Vec<Rc<RefCell<ActivityEdge>>>,
    in_partition: Vec<Rc<RefCell<ActivityPartition>>>,
    in_interruptible_region: Vec<Rc<RefCell<InterruptibleActivityRegion>>>,
    redefined_node: Vec<Rc<RefCell<ActivityNode>>>,
    handler: Vec<ExceptionHandler>,
    local_precondition: Vec<Constraint>,
    local_postcondition: Vec<Constraint>,
    variable: Rc<RefCell<Variable>>,
}

impl ClearVariableAction {
    pub fn new(is_leaf: bool, variable: Rc<RefCell<Variable>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            in_structured_node: None,
            activity: None,
            outgoing: Vec::new(),
            incoming: Vec::new(),
            in_partition: Vec::new(),
            in_interruptible_region: Vec::new(),
            redefined_node: Vec::new(),
            handler: Vec::new(),
            local_precondition: Vec::new(),
            local_postcondition: Vec::new(),
            variable: variable,
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

    /// Returns a reference to outgoing
    pub fn outgoing(&self) -> &Vec<Rc<RefCell<ActivityEdge>>> {
        &self.outgoing
    }

    /// Returns a mutable reference to outgoing
    pub fn outgoing_mut(&mut self) -> &mut Vec<Rc<RefCell<ActivityEdge>>> {
        &mut self.outgoing
    }

    /// Adds an item to outgoing
    pub fn add_outgoing(&mut self, item: Rc<RefCell<ActivityEdge>>) {
        self.outgoing.push(item);
    }

    /// Clears all items from outgoing
    pub fn clear_outgoing(&mut self) {
        self.outgoing.clear();
    }

    /// Returns a reference to incoming
    pub fn incoming(&self) -> &Vec<Rc<RefCell<ActivityEdge>>> {
        &self.incoming
    }

    /// Returns a mutable reference to incoming
    pub fn incoming_mut(&mut self) -> &mut Vec<Rc<RefCell<ActivityEdge>>> {
        &mut self.incoming
    }

    /// Adds an item to incoming
    pub fn add_incoming(&mut self, item: Rc<RefCell<ActivityEdge>>) {
        self.incoming.push(item);
    }

    /// Clears all items from incoming
    pub fn clear_incoming(&mut self) {
        self.incoming.clear();
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

    /// Returns a reference to in_interruptible_region
    pub fn in_interruptible_region(&self) -> &Vec<Rc<RefCell<InterruptibleActivityRegion>>> {
        &self.in_interruptible_region
    }

    /// Returns a mutable reference to in_interruptible_region
    pub fn in_interruptible_region_mut(&mut self) -> &mut Vec<Rc<RefCell<InterruptibleActivityRegion>>> {
        &mut self.in_interruptible_region
    }

    /// Adds an item to in_interruptible_region
    pub fn add_in_interruptible_region(&mut self, item: Rc<RefCell<InterruptibleActivityRegion>>) {
        self.in_interruptible_region.push(item);
    }

    /// Clears all items from in_interruptible_region
    pub fn clear_in_interruptible_region(&mut self) {
        self.in_interruptible_region.clear();
    }

    /// Returns a reference to redefined_node
    pub fn redefined_node(&self) -> &Vec<Rc<RefCell<ActivityNode>>> {
        &self.redefined_node
    }

    /// Returns a mutable reference to redefined_node
    pub fn redefined_node_mut(&mut self) -> &mut Vec<Rc<RefCell<ActivityNode>>> {
        &mut self.redefined_node
    }

    /// Adds an item to redefined_node
    pub fn add_redefined_node(&mut self, item: Rc<RefCell<ActivityNode>>) {
        self.redefined_node.push(item);
    }

    /// Clears all items from redefined_node
    pub fn clear_redefined_node(&mut self) {
        self.redefined_node.clear();
    }

    /// Returns a slice of handler
    pub fn handler(&self) -> &[ExceptionHandler] {
        &self.handler
    }

    /// Returns a mutable reference to handler
    pub fn handler_mut(&mut self) -> &mut Vec<ExceptionHandler> {
        &mut self.handler
    }

    /// Adds an item to handler
    pub fn add_handler(&mut self, item: ExceptionHandler) {
        self.handler.push(item);
    }

    /// Clears all items from handler
    pub fn clear_handler(&mut self) {
        self.handler.clear();
    }

    /// Returns a slice of local_precondition
    pub fn local_precondition(&self) -> &[Constraint] {
        &self.local_precondition
    }

    /// Returns a mutable reference to local_precondition
    pub fn local_precondition_mut(&mut self) -> &mut Vec<Constraint> {
        &mut self.local_precondition
    }

    /// Adds an item to local_precondition
    pub fn add_local_precondition(&mut self, item: Constraint) {
        self.local_precondition.push(item);
    }

    /// Clears all items from local_precondition
    pub fn clear_local_precondition(&mut self) {
        self.local_precondition.clear();
    }

    /// Returns a slice of local_postcondition
    pub fn local_postcondition(&self) -> &[Constraint] {
        &self.local_postcondition
    }

    /// Returns a mutable reference to local_postcondition
    pub fn local_postcondition_mut(&mut self) -> &mut Vec<Constraint> {
        &mut self.local_postcondition
    }

    /// Adds an item to local_postcondition
    pub fn add_local_postcondition(&mut self, item: Constraint) {
        self.local_postcondition.push(item);
    }

    /// Clears all items from local_postcondition
    pub fn clear_local_postcondition(&mut self) {
        self.local_postcondition.clear();
    }

    /// Returns a reference to variable
    pub fn variable(&self) -> &Rc<RefCell<Variable>> {
        &self.variable
    }

    /// Returns a mutable reference to variable
    pub fn variable_mut(&mut self) -> &mut Rc<RefCell<Variable>> {
        &mut self.variable
    }

    /// Sets variable
    pub fn set_variable(&mut self, value: Rc<RefCell<Variable>>) {
        self.variable = value;
    }

}

impl Default for ClearVariableAction {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: false,
            in_structured_node: None,
            activity: None,
            outgoing: Vec::new(),
            incoming: Vec::new(),
            in_partition: Vec::new(),
            in_interruptible_region: Vec::new(),
            redefined_node: Vec::new(),
            handler: Vec::new(),
            local_precondition: Vec::new(),
            local_postcondition: Vec::new(),
            variable: Default::default(),
        }
    }
}

impl std::fmt::Display for ClearVariableAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

