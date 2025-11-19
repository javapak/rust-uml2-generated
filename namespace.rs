use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::constraint::Constraint;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Namespace {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    element_import: Vec<ElementImport>,
    package_import: Vec<PackageImport>,
    owned_rule: Vec<Constraint>,
}

impl Namespace {
    pub fn new() -> Self {
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

}

