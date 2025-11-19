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
use crate::type_::Type;
use crate::object_node_ordering_kind::ObjectNodeOrderingKind;
use crate::value_specification::ValueSpecification;
use crate::state::State;
use crate::behavior::Behavior;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPin {
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
    type_: Option<Rc<RefCell<Type>>>,
    ordering: String,
    is_control_type: bool,
    upper_bound: ValueSpecification,
    in_state: Vec<Rc<RefCell<State>>>,
    selection: Option<Rc<RefCell<Behavior>>>,
    is_ordered: bool,
    is_unique: bool,
    upper_value: Option<ValueSpecification>,
    lower_value: Option<ValueSpecification>,
    is_control: bool,
}

impl InputPin {
    pub fn new(is_leaf: bool, ordering: String, is_control_type: bool, upper_bound: ValueSpecification, is_ordered: bool, is_unique: bool, is_control: bool) -> Self {
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
            type_: None,
            ordering: ordering,
            is_control_type: is_control_type,
            upper_bound: upper_bound,
            in_state: Vec::new(),
            selection: None,
            is_ordered: is_ordered,
            is_unique: is_unique,
            upper_value: None,
            lower_value: None,
            is_control: is_control,
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

    /// Returns a reference to type_ if present
    pub fn type_(&self) -> Option<&Rc<RefCell<Type>>> {
        self.type_.as_ref()
    }

    /// Returns a mutable reference to type_ if present
    pub fn type_mut(&mut self) -> Option<&mut Rc<RefCell<Type>>> {
        self.type_.as_mut()
    }

    /// Sets type_
    pub fn set_type_(&mut self, value: Rc<RefCell<Type>>) {
        self.type_ = Some(value);
    }

    /// Takes type_, leaving None in its place
    pub fn take_type(&mut self) -> Option<Rc<RefCell<Type>>> {
        self.type_.take()
    }

    /// Returns ordering as a string slice
    pub fn ordering(&self) -> &str {
        &self.ordering
    }

    /// Sets ordering
    pub fn set_ordering(&mut self, value: impl Into<String>) {
        self.ordering = value.into();
    }

    /// Takes ownership of ordering, replacing it with an empty string
    pub fn take_ordering(&mut self) -> String {
        std::mem::take(&mut self.ordering)
    }

    /// Returns is_control_type
    pub fn is_control_type(&self) -> bool {
        self.is_control_type
    }

    /// Sets is_control_type
    pub fn set_is_control_type(&mut self, value: bool) {
        self.is_control_type = value;
    }

    /// Returns a reference to upper_bound
    pub fn upper_bound(&self) -> &ValueSpecification {
        &self.upper_bound
    }

    /// Returns a mutable reference to upper_bound
    pub fn upper_bound_mut(&mut self) -> &mut ValueSpecification {
        &mut self.upper_bound
    }

    /// Sets upper_bound
    pub fn set_upper_bound(&mut self, value: ValueSpecification) {
        self.upper_bound = value;
    }

    /// Returns a reference to in_state
    pub fn in_state(&self) -> &Vec<Rc<RefCell<State>>> {
        &self.in_state
    }

    /// Returns a mutable reference to in_state
    pub fn in_state_mut(&mut self) -> &mut Vec<Rc<RefCell<State>>> {
        &mut self.in_state
    }

    /// Adds an item to in_state
    pub fn add_in_state(&mut self, item: Rc<RefCell<State>>) {
        self.in_state.push(item);
    }

    /// Clears all items from in_state
    pub fn clear_in_state(&mut self) {
        self.in_state.clear();
    }

    /// Returns a reference to selection if present
    pub fn selection(&self) -> Option<&Rc<RefCell<Behavior>>> {
        self.selection.as_ref()
    }

    /// Returns a mutable reference to selection if present
    pub fn selection_mut(&mut self) -> Option<&mut Rc<RefCell<Behavior>>> {
        self.selection.as_mut()
    }

    /// Sets selection
    pub fn set_selection(&mut self, value: Rc<RefCell<Behavior>>) {
        self.selection = Some(value);
    }

    /// Takes selection, leaving None in its place
    pub fn take_selection(&mut self) -> Option<Rc<RefCell<Behavior>>> {
        self.selection.take()
    }

    /// Returns is_ordered
    pub fn is_ordered(&self) -> bool {
        self.is_ordered
    }

    /// Sets is_ordered
    pub fn set_is_ordered(&mut self, value: bool) {
        self.is_ordered = value;
    }

    /// Returns is_unique
    pub fn is_unique(&self) -> bool {
        self.is_unique
    }

    /// Sets is_unique
    pub fn set_is_unique(&mut self, value: bool) {
        self.is_unique = value;
    }

    /// Returns a reference to upper_value if present
    pub fn upper_value(&self) -> Option<&ValueSpecification> {
        self.upper_value.as_ref()
    }

    /// Returns a mutable reference to upper_value if present
    pub fn upper_value_mut(&mut self) -> Option<&mut ValueSpecification> {
        self.upper_value.as_mut()
    }

    /// Sets upper_value
    pub fn set_upper_value(&mut self, value: ValueSpecification) {
        self.upper_value = Some(value);
    }

    /// Takes upper_value, leaving None in its place
    pub fn take_upper_value(&mut self) -> Option<ValueSpecification> {
        self.upper_value.take()
    }

    /// Returns a reference to lower_value if present
    pub fn lower_value(&self) -> Option<&ValueSpecification> {
        self.lower_value.as_ref()
    }

    /// Returns a mutable reference to lower_value if present
    pub fn lower_value_mut(&mut self) -> Option<&mut ValueSpecification> {
        self.lower_value.as_mut()
    }

    /// Sets lower_value
    pub fn set_lower_value(&mut self, value: ValueSpecification) {
        self.lower_value = Some(value);
    }

    /// Takes lower_value, leaving None in its place
    pub fn take_lower_value(&mut self) -> Option<ValueSpecification> {
        self.lower_value.take()
    }

    /// Returns is_control
    pub fn is_control(&self) -> bool {
        self.is_control
    }

    /// Sets is_control
    pub fn set_is_control(&mut self, value: bool) {
        self.is_control = value;
    }

}

impl Default for InputPin {
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
            type_: None,
            ordering: String::new(),
            is_control_type: false,
            upper_bound: Default::default(),
            in_state: Vec::new(),
            selection: None,
            is_ordered: false,
            is_unique: false,
            upper_value: None,
            lower_value: None,
            is_control: false,
        }
    }
}

impl std::fmt::Display for InputPin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

