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
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::variable::Variable;
use crate::executable_node::ExecutableNode;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SequenceNode {
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
    element_import: Vec<ElementImport>,
    package_import: Vec<PackageImport>,
    owned_rule: Vec<Constraint>,
    in_activity: Option<Weak<RefCell<Activity>>>,
    variable: Vec<Variable>,
    edge: Vec<ActivityEdge>,
    must_isolate: bool,
    node: Vec<ActivityNode>,
    executable_node: Vec<ExecutableNode>,
}

impl SequenceNode {
    pub fn new(is_leaf: bool, must_isolate: bool) -> Self {
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
            element_import: Vec::new(),
            package_import: Vec::new(),
            owned_rule: Vec::new(),
            in_activity: None,
            variable: Vec::new(),
            edge: Vec::new(),
            must_isolate: must_isolate,
            node: Vec::new(),
            executable_node: Vec::new(),
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

    /// Returns a slice of element_import
    pub fn element_import(&self) -> &[ElementImport] {
        &self.element_import
    }

    /// Returns a mutable reference to element_import
    pub fn element_import_mut(&mut self) -> &mut Vec<ElementImport> {
        &mut self.element_import
    }

    /// Adds an item to element_import
    pub fn add_element_import(&mut self, item: ElementImport) {
        self.element_import.push(item);
    }

    /// Clears all items from element_import
    pub fn clear_element_import(&mut self) {
        self.element_import.clear();
    }

    /// Returns a slice of package_import
    pub fn package_import(&self) -> &[PackageImport] {
        &self.package_import
    }

    /// Returns a mutable reference to package_import
    pub fn package_import_mut(&mut self) -> &mut Vec<PackageImport> {
        &mut self.package_import
    }

    /// Adds an item to package_import
    pub fn add_package_import(&mut self, item: PackageImport) {
        self.package_import.push(item);
    }

    /// Clears all items from package_import
    pub fn clear_package_import(&mut self) {
        self.package_import.clear();
    }

    /// Returns a slice of owned_rule
    pub fn owned_rule(&self) -> &[Constraint] {
        &self.owned_rule
    }

    /// Returns a mutable reference to owned_rule
    pub fn owned_rule_mut(&mut self) -> &mut Vec<Constraint> {
        &mut self.owned_rule
    }

    /// Adds an item to owned_rule
    pub fn add_owned_rule(&mut self, item: Constraint) {
        self.owned_rule.push(item);
    }

    /// Clears all items from owned_rule
    pub fn clear_owned_rule(&mut self) {
        self.owned_rule.clear();
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

    /// Returns a slice of variable
    pub fn variable(&self) -> &[Variable] {
        &self.variable
    }

    /// Returns a mutable reference to variable
    pub fn variable_mut(&mut self) -> &mut Vec<Variable> {
        &mut self.variable
    }

    /// Adds an item to variable
    pub fn add_variable(&mut self, item: Variable) {
        self.variable.push(item);
    }

    /// Clears all items from variable
    pub fn clear_variable(&mut self) {
        self.variable.clear();
    }

    /// Returns a slice of edge
    pub fn edge(&self) -> &[ActivityEdge] {
        &self.edge
    }

    /// Returns a mutable reference to edge
    pub fn edge_mut(&mut self) -> &mut Vec<ActivityEdge> {
        &mut self.edge
    }

    /// Adds an item to edge
    pub fn add_edge(&mut self, item: ActivityEdge) {
        self.edge.push(item);
    }

    /// Clears all items from edge
    pub fn clear_edge(&mut self) {
        self.edge.clear();
    }

    /// Returns must_isolate
    pub fn must_isolate(&self) -> bool {
        self.must_isolate
    }

    /// Sets must_isolate
    pub fn set_must_isolate(&mut self, value: bool) {
        self.must_isolate = value;
    }

    /// Returns a slice of node
    pub fn node(&self) -> &[ActivityNode] {
        &self.node
    }

    /// Returns a mutable reference to node
    pub fn node_mut(&mut self) -> &mut Vec<ActivityNode> {
        &mut self.node
    }

    /// Adds an item to node
    pub fn add_node(&mut self, item: ActivityNode) {
        self.node.push(item);
    }

    /// Clears all items from node
    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    /// Returns a slice of executable_node
    pub fn executable_node(&self) -> &[ExecutableNode] {
        &self.executable_node
    }

    /// Returns a mutable reference to executable_node
    pub fn executable_node_mut(&mut self) -> &mut Vec<ExecutableNode> {
        &mut self.executable_node
    }

    /// Adds an item to executable_node
    pub fn add_executable_node(&mut self, item: ExecutableNode) {
        self.executable_node.push(item);
    }

    /// Clears all items from executable_node
    pub fn clear_executable_node(&mut self) {
        self.executable_node.clear();
    }

}

impl std::fmt::Display for SequenceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

