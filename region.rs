use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::constraint::Constraint;
use crate::vertex::Vertex;
use crate::transition::Transition;
use crate::state::State;
use crate::state_machine::StateMachine;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Region {
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
    subvertex: Vec<Vertex>,
    transition: Vec<Transition>,
    state: Option<Weak<RefCell<State>>>,
    extended_region: Option<Rc<RefCell<Region>>>,
    state_machine: Option<Weak<RefCell<StateMachine>>>,
}

impl Region {
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
            subvertex: Vec::new(),
            transition: Vec::new(),
            state: None,
            extended_region: None,
            state_machine: None,
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

    /// Returns a slice of subvertex
    pub fn subvertex(&self) -> &[Vertex] {
        &self.subvertex
    }

    /// Returns a mutable reference to subvertex
    pub fn subvertex_mut(&mut self) -> &mut Vec<Vertex> {
        &mut self.subvertex
    }

    /// Adds an item to subvertex
    pub fn add_subvertex(&mut self, item: Vertex) {
        self.subvertex.push(item);
    }

    /// Clears all items from subvertex
    pub fn clear_subvertex(&mut self) {
        self.subvertex.clear();
    }

    /// Returns a slice of transition
    pub fn transition(&self) -> &[Transition] {
        &self.transition
    }

    /// Returns a mutable reference to transition
    pub fn transition_mut(&mut self) -> &mut Vec<Transition> {
        &mut self.transition
    }

    /// Adds an item to transition
    pub fn add_transition(&mut self, item: Transition) {
        self.transition.push(item);
    }

    /// Clears all items from transition
    pub fn clear_transition(&mut self) {
        self.transition.clear();
    }

    /// Returns a reference to state if present
    pub fn state(&self) -> Option<&Weak<RefCell<State>>> {
        self.state.as_ref()
    }

    /// Returns a mutable reference to state if present
    pub fn state_mut(&mut self) -> Option<&mut Weak<RefCell<State>>> {
        self.state.as_mut()
    }

    /// Sets state
    pub fn set_state(&mut self, value: Weak<RefCell<State>>) {
        self.state = Some(value);
    }

    /// Takes state, leaving None in its place
    pub fn take_state(&mut self) -> Option<Weak<RefCell<State>>> {
        self.state.take()
    }

    /// Returns a reference to extended_region if present
    pub fn extended_region(&self) -> Option<&Rc<RefCell<Region>>> {
        self.extended_region.as_ref()
    }

    /// Returns a mutable reference to extended_region if present
    pub fn extended_region_mut(&mut self) -> Option<&mut Rc<RefCell<Region>>> {
        self.extended_region.as_mut()
    }

    /// Sets extended_region
    pub fn set_extended_region(&mut self, value: Rc<RefCell<Region>>) {
        self.extended_region = Some(value);
    }

    /// Takes extended_region, leaving None in its place
    pub fn take_extended_region(&mut self) -> Option<Rc<RefCell<Region>>> {
        self.extended_region.take()
    }

    /// Returns a reference to state_machine if present
    pub fn state_machine(&self) -> Option<&Weak<RefCell<StateMachine>>> {
        self.state_machine.as_ref()
    }

    /// Returns a mutable reference to state_machine if present
    pub fn state_machine_mut(&mut self) -> Option<&mut Weak<RefCell<StateMachine>>> {
        self.state_machine.as_mut()
    }

    /// Sets state_machine
    pub fn set_state_machine(&mut self, value: Weak<RefCell<StateMachine>>) {
        self.state_machine = Some(value);
    }

    /// Takes state_machine, leaving None in its place
    pub fn take_state_machine(&mut self) -> Option<Weak<RefCell<StateMachine>>> {
        self.state_machine.take()
    }

}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

