use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::constraint::Constraint;
use crate::transition_kind::TransitionKind;
use crate::region::Region;
use crate::transition::Transition;
use crate::behavior::Behavior;
use crate::trigger::Trigger;
use crate::vertex::Vertex;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolTransition {
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
    kind: String,
    container: Weak<RefCell<Region>>,
    redefined_transition: Option<Rc<RefCell<Transition>>>,
    guard: Option<Rc<RefCell<Constraint>>>,
    effect: Option<Behavior>,
    trigger: Vec<Trigger>,
    target: Rc<RefCell<Vertex>>,
    source: Rc<RefCell<Vertex>>,
    post_condition: Option<Rc<RefCell<Constraint>>>,
    pre_condition: Option<Rc<RefCell<Constraint>>>,
}

impl ProtocolTransition {
    pub fn new(is_leaf: bool, kind: String, container: Weak<RefCell<Region>>, target: Rc<RefCell<Vertex>>, source: Rc<RefCell<Vertex>>) -> Self {
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
            kind: kind,
            container: container,
            redefined_transition: None,
            guard: None,
            effect: None,
            trigger: Vec::new(),
            target: target,
            source: source,
            post_condition: None,
            pre_condition: None,
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

    /// Returns kind as a string slice
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Sets kind
    pub fn set_kind(&mut self, value: impl Into<String>) {
        self.kind = value.into();
    }

    /// Takes ownership of kind, replacing it with an empty string
    pub fn take_kind(&mut self) -> String {
        std::mem::take(&mut self.kind)
    }

    /// Returns a reference to container
    pub fn container(&self) -> &Weak<RefCell<Region>> {
        &self.container
    }

    /// Returns a mutable reference to container
    pub fn container_mut(&mut self) -> &mut Weak<RefCell<Region>> {
        &mut self.container
    }

    /// Sets container
    pub fn set_container(&mut self, value: Weak<RefCell<Region>>) {
        self.container = value;
    }

    /// Returns a reference to redefined_transition if present
    pub fn redefined_transition(&self) -> Option<&Rc<RefCell<Transition>>> {
        self.redefined_transition.as_ref()
    }

    /// Returns a mutable reference to redefined_transition if present
    pub fn redefined_transition_mut(&mut self) -> Option<&mut Rc<RefCell<Transition>>> {
        self.redefined_transition.as_mut()
    }

    /// Sets redefined_transition
    pub fn set_redefined_transition(&mut self, value: Rc<RefCell<Transition>>) {
        self.redefined_transition = Some(value);
    }

    /// Takes redefined_transition, leaving None in its place
    pub fn take_redefined_transition(&mut self) -> Option<Rc<RefCell<Transition>>> {
        self.redefined_transition.take()
    }

    /// Returns a reference to guard if present
    pub fn guard(&self) -> Option<&Rc<RefCell<Constraint>>> {
        self.guard.as_ref()
    }

    /// Returns a mutable reference to guard if present
    pub fn guard_mut(&mut self) -> Option<&mut Rc<RefCell<Constraint>>> {
        self.guard.as_mut()
    }

    /// Sets guard
    pub fn set_guard(&mut self, value: Rc<RefCell<Constraint>>) {
        self.guard = Some(value);
    }

    /// Takes guard, leaving None in its place
    pub fn take_guard(&mut self) -> Option<Rc<RefCell<Constraint>>> {
        self.guard.take()
    }

    /// Returns a reference to effect if present
    pub fn effect(&self) -> Option<&Behavior> {
        self.effect.as_ref()
    }

    /// Returns a mutable reference to effect if present
    pub fn effect_mut(&mut self) -> Option<&mut Behavior> {
        self.effect.as_mut()
    }

    /// Sets effect
    pub fn set_effect(&mut self, value: Behavior) {
        self.effect = Some(value);
    }

    /// Takes effect, leaving None in its place
    pub fn take_effect(&mut self) -> Option<Behavior> {
        self.effect.take()
    }

    /// Returns a slice of trigger
    pub fn trigger(&self) -> &[Trigger] {
        &self.trigger
    }

    /// Returns a mutable reference to trigger
    pub fn trigger_mut(&mut self) -> &mut Vec<Trigger> {
        &mut self.trigger
    }

    /// Adds an item to trigger
    pub fn add_trigger(&mut self, item: Trigger) {
        self.trigger.push(item);
    }

    /// Clears all items from trigger
    pub fn clear_trigger(&mut self) {
        self.trigger.clear();
    }

    /// Returns a reference to target
    pub fn target(&self) -> &Rc<RefCell<Vertex>> {
        &self.target
    }

    /// Returns a mutable reference to target
    pub fn target_mut(&mut self) -> &mut Rc<RefCell<Vertex>> {
        &mut self.target
    }

    /// Sets target
    pub fn set_target(&mut self, value: Rc<RefCell<Vertex>>) {
        self.target = value;
    }

    /// Returns a reference to source
    pub fn source(&self) -> &Rc<RefCell<Vertex>> {
        &self.source
    }

    /// Returns a mutable reference to source
    pub fn source_mut(&mut self) -> &mut Rc<RefCell<Vertex>> {
        &mut self.source
    }

    /// Sets source
    pub fn set_source(&mut self, value: Rc<RefCell<Vertex>>) {
        self.source = value;
    }

    /// Returns a reference to post_condition if present
    pub fn post_condition(&self) -> Option<&Rc<RefCell<Constraint>>> {
        self.post_condition.as_ref()
    }

    /// Returns a mutable reference to post_condition if present
    pub fn post_condition_mut(&mut self) -> Option<&mut Rc<RefCell<Constraint>>> {
        self.post_condition.as_mut()
    }

    /// Sets post_condition
    pub fn set_post_condition(&mut self, value: Rc<RefCell<Constraint>>) {
        self.post_condition = Some(value);
    }

    /// Takes post_condition, leaving None in its place
    pub fn take_post_condition(&mut self) -> Option<Rc<RefCell<Constraint>>> {
        self.post_condition.take()
    }

    /// Returns a reference to pre_condition if present
    pub fn pre_condition(&self) -> Option<&Rc<RefCell<Constraint>>> {
        self.pre_condition.as_ref()
    }

    /// Returns a mutable reference to pre_condition if present
    pub fn pre_condition_mut(&mut self) -> Option<&mut Rc<RefCell<Constraint>>> {
        self.pre_condition.as_mut()
    }

    /// Sets pre_condition
    pub fn set_pre_condition(&mut self, value: Rc<RefCell<Constraint>>) {
        self.pre_condition = Some(value);
    }

    /// Takes pre_condition, leaving None in its place
    pub fn take_pre_condition(&mut self) -> Option<Rc<RefCell<Constraint>>> {
        self.pre_condition.take()
    }

}

impl Default for ProtocolTransition {
    fn default() -> Self {
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
            is_leaf: false,
            kind: String::new(),
            container: Default::default(),
            redefined_transition: None,
            guard: None,
            effect: None,
            trigger: Vec::new(),
            target: Default::default(),
            source: Default::default(),
            post_condition: None,
            pre_condition: None,
        }
    }
}

impl std::fmt::Display for ProtocolTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

