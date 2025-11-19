use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::connectable_element::ConnectableElement;
use crate::interaction::Interaction;
use crate::value_specification::ValueSpecification;
use crate::part_decomposition::PartDecomposition;
use crate::interaction_fragment::InteractionFragment;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lifeline {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    represents: Option<Rc<RefCell<ConnectableElement>>>,
    interaction: Weak<RefCell<Interaction>>,
    selector: Option<ValueSpecification>,
    decomposed_as: Option<Rc<RefCell<PartDecomposition>>>,
    covered_by: Vec<Rc<RefCell<InteractionFragment>>>,
}

impl Lifeline {
    pub fn new(interaction: Weak<RefCell<Interaction>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            represents: None,
            interaction: interaction,
            selector: None,
            decomposed_as: None,
            covered_by: Vec::new(),
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

    /// Returns a reference to represents if present
    pub fn represents(&self) -> Option<&Rc<RefCell<ConnectableElement>>> {
        self.represents.as_ref()
    }

    /// Returns a mutable reference to represents if present
    pub fn represents_mut(&mut self) -> Option<&mut Rc<RefCell<ConnectableElement>>> {
        self.represents.as_mut()
    }

    /// Sets represents
    pub fn set_represents(&mut self, value: Rc<RefCell<ConnectableElement>>) {
        self.represents = Some(value);
    }

    /// Takes represents, leaving None in its place
    pub fn take_represents(&mut self) -> Option<Rc<RefCell<ConnectableElement>>> {
        self.represents.take()
    }

    /// Returns a reference to interaction
    pub fn interaction(&self) -> &Weak<RefCell<Interaction>> {
        &self.interaction
    }

    /// Returns a mutable reference to interaction
    pub fn interaction_mut(&mut self) -> &mut Weak<RefCell<Interaction>> {
        &mut self.interaction
    }

    /// Sets interaction
    pub fn set_interaction(&mut self, value: Weak<RefCell<Interaction>>) {
        self.interaction = value;
    }

    /// Returns a reference to selector if present
    pub fn selector(&self) -> Option<&ValueSpecification> {
        self.selector.as_ref()
    }

    /// Returns a mutable reference to selector if present
    pub fn selector_mut(&mut self) -> Option<&mut ValueSpecification> {
        self.selector.as_mut()
    }

    /// Sets selector
    pub fn set_selector(&mut self, value: ValueSpecification) {
        self.selector = Some(value);
    }

    /// Takes selector, leaving None in its place
    pub fn take_selector(&mut self) -> Option<ValueSpecification> {
        self.selector.take()
    }

    /// Returns a reference to decomposed_as if present
    pub fn decomposed_as(&self) -> Option<&Rc<RefCell<PartDecomposition>>> {
        self.decomposed_as.as_ref()
    }

    /// Returns a mutable reference to decomposed_as if present
    pub fn decomposed_as_mut(&mut self) -> Option<&mut Rc<RefCell<PartDecomposition>>> {
        self.decomposed_as.as_mut()
    }

    /// Sets decomposed_as
    pub fn set_decomposed_as(&mut self, value: Rc<RefCell<PartDecomposition>>) {
        self.decomposed_as = Some(value);
    }

    /// Takes decomposed_as, leaving None in its place
    pub fn take_decomposed_as(&mut self) -> Option<Rc<RefCell<PartDecomposition>>> {
        self.decomposed_as.take()
    }

    /// Returns a reference to covered_by
    pub fn covered_by(&self) -> &Vec<Rc<RefCell<InteractionFragment>>> {
        &self.covered_by
    }

    /// Returns a mutable reference to covered_by
    pub fn covered_by_mut(&mut self) -> &mut Vec<Rc<RefCell<InteractionFragment>>> {
        &mut self.covered_by
    }

    /// Adds an item to covered_by
    pub fn add_covered_by(&mut self, item: Rc<RefCell<InteractionFragment>>) {
        self.covered_by.push(item);
    }

    /// Clears all items from covered_by
    pub fn clear_covered_by(&mut self) {
        self.covered_by.clear();
    }

}

impl Default for Lifeline {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            represents: None,
            interaction: Default::default(),
            selector: None,
            decomposed_as: None,
            covered_by: Vec::new(),
        }
    }
}

impl std::fmt::Display for Lifeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

