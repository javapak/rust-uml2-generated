use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::template_parameter::TemplateParameter;
use crate::templateable_element::TemplateableElement;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedefinableTemplateSignature {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    parameter: Vec<Rc<RefCell<TemplateParameter>>>,
    owned_parameter: Vec<TemplateParameter>,
    template: Weak<RefCell<TemplateableElement>>,
    extended_signature: Vec<Rc<RefCell<RedefinableTemplateSignature>>>,
}

impl RedefinableTemplateSignature {
    pub fn new(is_leaf: bool, parameter: Vec<Rc<RefCell<TemplateParameter>>>, template: Weak<RefCell<TemplateableElement>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            parameter: parameter,
            owned_parameter: Vec::new(),
            template: template,
            extended_signature: Vec::new(),
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

    /// Returns a reference to parameter
    pub fn parameter(&self) -> &Vec<Rc<RefCell<TemplateParameter>>> {
        &self.parameter
    }

    /// Returns a mutable reference to parameter
    pub fn parameter_mut(&mut self) -> &mut Vec<Rc<RefCell<TemplateParameter>>> {
        &mut self.parameter
    }

    /// Adds an item to parameter
    pub fn add_parameter(&mut self, item: Rc<RefCell<TemplateParameter>>) {
        self.parameter.push(item);
    }

    /// Clears all items from parameter
    pub fn clear_parameter(&mut self) {
        self.parameter.clear();
    }

    /// Returns a slice of owned_parameter
    pub fn owned_parameter(&self) -> &[TemplateParameter] {
        &self.owned_parameter
    }

    /// Returns a mutable reference to owned_parameter
    pub fn owned_parameter_mut(&mut self) -> &mut Vec<TemplateParameter> {
        &mut self.owned_parameter
    }

    /// Adds an item to owned_parameter
    pub fn add_owned_parameter(&mut self, item: TemplateParameter) {
        self.owned_parameter.push(item);
    }

    /// Clears all items from owned_parameter
    pub fn clear_owned_parameter(&mut self) {
        self.owned_parameter.clear();
    }

    /// Returns a reference to template
    pub fn template(&self) -> &Weak<RefCell<TemplateableElement>> {
        &self.template
    }

    /// Returns a mutable reference to template
    pub fn template_mut(&mut self) -> &mut Weak<RefCell<TemplateableElement>> {
        &mut self.template
    }

    /// Sets template
    pub fn set_template(&mut self, value: Weak<RefCell<TemplateableElement>>) {
        self.template = value;
    }

    /// Returns a reference to extended_signature
    pub fn extended_signature(&self) -> &Vec<Rc<RefCell<RedefinableTemplateSignature>>> {
        &self.extended_signature
    }

    /// Returns a mutable reference to extended_signature
    pub fn extended_signature_mut(&mut self) -> &mut Vec<Rc<RefCell<RedefinableTemplateSignature>>> {
        &mut self.extended_signature
    }

    /// Adds an item to extended_signature
    pub fn add_extended_signature(&mut self, item: Rc<RefCell<RedefinableTemplateSignature>>) {
        self.extended_signature.push(item);
    }

    /// Clears all items from extended_signature
    pub fn clear_extended_signature(&mut self) {
        self.extended_signature.clear();
    }

}

impl Default for RedefinableTemplateSignature {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: false,
            parameter: Vec::new(),
            owned_parameter: Vec::new(),
            template: Default::default(),
            extended_signature: Vec::new(),
        }
    }
}

impl std::fmt::Display for RedefinableTemplateSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

