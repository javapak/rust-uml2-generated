use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::constraint::Constraint;
use crate::region::Region;
use crate::state_machine::StateMachine;
use crate::connection_point_reference::ConnectionPointReference;
use crate::pseudostate::Pseudostate;
use crate::behavior::Behavior;
use crate::trigger::Trigger;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct State {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    element_import: Vec<ElementImport>,
    package_import: Vec<PackageImport>,
    owned_rule: Vec<Constraint>,
    is_leaf: bool,
    container: Option<Weak<RefCell<Region>>>,
    submachine: Option<Rc<RefCell<StateMachine>>>,
    connection: Vec<ConnectionPointReference>,
    connection_point: Vec<Pseudostate>,
    redefined_state: Option<Rc<RefCell<State>>>,
    state_invariant: Option<Constraint>,
    entry: Option<Behavior>,
    exit: Option<Behavior>,
    do_activity: Option<Behavior>,
    deferrable_trigger: Vec<Trigger>,
    region: Vec<Region>,
}

impl State {
    pub fn new(is_leaf: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            element_import: Vec::new(),
            package_import: Vec::new(),
            owned_rule: Vec::new(),
            is_leaf: is_leaf,
            container: None,
            submachine: None,
            connection: Vec::new(),
            connection_point: Vec::new(),
            redefined_state: None,
            state_invariant: None,
            entry: None,
            exit: None,
            do_activity: None,
            deferrable_trigger: Vec::new(),
            region: Vec::new(),
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

    /// Returns is_leaf
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    /// Sets is_leaf
    pub fn set_is_leaf(&mut self, value: bool) {
        self.is_leaf = value;
    }

    /// Returns a reference to container if present
    pub fn container(&self) -> Option<&Weak<RefCell<Region>>> {
        self.container.as_ref()
    }

    /// Returns a mutable reference to container if present
    pub fn container_mut(&mut self) -> Option<&mut Weak<RefCell<Region>>> {
        self.container.as_mut()
    }

    /// Sets container
    pub fn set_container(&mut self, value: Weak<RefCell<Region>>) {
        self.container = Some(value);
    }

    /// Takes container, leaving None in its place
    pub fn take_container(&mut self) -> Option<Weak<RefCell<Region>>> {
        self.container.take()
    }

    /// Returns a reference to submachine if present
    pub fn submachine(&self) -> Option<&Rc<RefCell<StateMachine>>> {
        self.submachine.as_ref()
    }

    /// Returns a mutable reference to submachine if present
    pub fn submachine_mut(&mut self) -> Option<&mut Rc<RefCell<StateMachine>>> {
        self.submachine.as_mut()
    }

    /// Sets submachine
    pub fn set_submachine(&mut self, value: Rc<RefCell<StateMachine>>) {
        self.submachine = Some(value);
    }

    /// Takes submachine, leaving None in its place
    pub fn take_submachine(&mut self) -> Option<Rc<RefCell<StateMachine>>> {
        self.submachine.take()
    }

    /// Returns a slice of connection
    pub fn connection(&self) -> &[ConnectionPointReference] {
        &self.connection
    }

    /// Returns a mutable reference to connection
    pub fn connection_mut(&mut self) -> &mut Vec<ConnectionPointReference> {
        &mut self.connection
    }

    /// Adds an item to connection
    pub fn add_connection(&mut self, item: ConnectionPointReference) {
        self.connection.push(item);
    }

    /// Clears all items from connection
    pub fn clear_connection(&mut self) {
        self.connection.clear();
    }

    /// Returns a slice of connection_point
    pub fn connection_point(&self) -> &[Pseudostate] {
        &self.connection_point
    }

    /// Returns a mutable reference to connection_point
    pub fn connection_point_mut(&mut self) -> &mut Vec<Pseudostate> {
        &mut self.connection_point
    }

    /// Adds an item to connection_point
    pub fn add_connection_point(&mut self, item: Pseudostate) {
        self.connection_point.push(item);
    }

    /// Clears all items from connection_point
    pub fn clear_connection_point(&mut self) {
        self.connection_point.clear();
    }

    /// Returns a reference to redefined_state if present
    pub fn redefined_state(&self) -> Option<&Rc<RefCell<State>>> {
        self.redefined_state.as_ref()
    }

    /// Returns a mutable reference to redefined_state if present
    pub fn redefined_state_mut(&mut self) -> Option<&mut Rc<RefCell<State>>> {
        self.redefined_state.as_mut()
    }

    /// Sets redefined_state
    pub fn set_redefined_state(&mut self, value: Rc<RefCell<State>>) {
        self.redefined_state = Some(value);
    }

    /// Takes redefined_state, leaving None in its place
    pub fn take_redefined_state(&mut self) -> Option<Rc<RefCell<State>>> {
        self.redefined_state.take()
    }

    /// Returns a reference to state_invariant if present
    pub fn state_invariant(&self) -> Option<&Constraint> {
        self.state_invariant.as_ref()
    }

    /// Returns a mutable reference to state_invariant if present
    pub fn state_invariant_mut(&mut self) -> Option<&mut Constraint> {
        self.state_invariant.as_mut()
    }

    /// Sets state_invariant
    pub fn set_state_invariant(&mut self, value: Constraint) {
        self.state_invariant = Some(value);
    }

    /// Takes state_invariant, leaving None in its place
    pub fn take_state_invariant(&mut self) -> Option<Constraint> {
        self.state_invariant.take()
    }

    /// Returns a reference to entry if present
    pub fn entry(&self) -> Option<&Behavior> {
        self.entry.as_ref()
    }

    /// Returns a mutable reference to entry if present
    pub fn entry_mut(&mut self) -> Option<&mut Behavior> {
        self.entry.as_mut()
    }

    /// Sets entry
    pub fn set_entry(&mut self, value: Behavior) {
        self.entry = Some(value);
    }

    /// Takes entry, leaving None in its place
    pub fn take_entry(&mut self) -> Option<Behavior> {
        self.entry.take()
    }

    /// Returns a reference to exit if present
    pub fn exit(&self) -> Option<&Behavior> {
        self.exit.as_ref()
    }

    /// Returns a mutable reference to exit if present
    pub fn exit_mut(&mut self) -> Option<&mut Behavior> {
        self.exit.as_mut()
    }

    /// Sets exit
    pub fn set_exit(&mut self, value: Behavior) {
        self.exit = Some(value);
    }

    /// Takes exit, leaving None in its place
    pub fn take_exit(&mut self) -> Option<Behavior> {
        self.exit.take()
    }

    /// Returns a reference to do_activity if present
    pub fn do_activity(&self) -> Option<&Behavior> {
        self.do_activity.as_ref()
    }

    /// Returns a mutable reference to do_activity if present
    pub fn do_activity_mut(&mut self) -> Option<&mut Behavior> {
        self.do_activity.as_mut()
    }

    /// Sets do_activity
    pub fn set_do_activity(&mut self, value: Behavior) {
        self.do_activity = Some(value);
    }

    /// Takes do_activity, leaving None in its place
    pub fn take_do_activity(&mut self) -> Option<Behavior> {
        self.do_activity.take()
    }

    /// Returns a slice of deferrable_trigger
    pub fn deferrable_trigger(&self) -> &[Trigger] {
        &self.deferrable_trigger
    }

    /// Returns a mutable reference to deferrable_trigger
    pub fn deferrable_trigger_mut(&mut self) -> &mut Vec<Trigger> {
        &mut self.deferrable_trigger
    }

    /// Adds an item to deferrable_trigger
    pub fn add_deferrable_trigger(&mut self, item: Trigger) {
        self.deferrable_trigger.push(item);
    }

    /// Clears all items from deferrable_trigger
    pub fn clear_deferrable_trigger(&mut self) {
        self.deferrable_trigger.clear();
    }

    /// Returns a slice of region
    pub fn region(&self) -> &[Region] {
        &self.region
    }

    /// Returns a mutable reference to region
    pub fn region_mut(&mut self) -> &mut Vec<Region> {
        &mut self.region
    }

    /// Adds an item to region
    pub fn add_region(&mut self, item: Region) {
        self.region.push(item);
    }

    /// Clears all items from region
    pub fn clear_region(&mut self) {
        self.region.clear();
    }

}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

