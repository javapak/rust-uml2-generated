use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::constraint::Constraint;
use crate::parameter::Parameter;
use crate::behavior::Behavior;
use crate::call_concurrency_kind::CallConcurrencyKind;
use crate::type_::Type;
use crate::parameter_set::ParameterSet;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BehavioralFeature {
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
    is_static: bool,
    owned_parameter: Vec<Parameter>,
    is_abstract: bool,
    method: Vec<Rc<RefCell<Behavior>>>,
    concurrency: String,
    raised_exception: Vec<Rc<RefCell<Type>>>,
    owned_parameter_set: Vec<ParameterSet>,
}

impl BehavioralFeature {
    pub fn new(is_leaf: bool, is_static: bool, is_abstract: bool, concurrency: String) -> Self {
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
            is_static: is_static,
            owned_parameter: Vec::new(),
            is_abstract: is_abstract,
            method: Vec::new(),
            concurrency: concurrency,
            raised_exception: Vec::new(),
            owned_parameter_set: Vec::new(),
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

    /// Returns is_static
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    /// Sets is_static
    pub fn set_is_static(&mut self, value: bool) {
        self.is_static = value;
    }

    /// Returns a slice of owned_parameter
    pub fn owned_parameter(&self) -> &[Parameter] {
        &self.owned_parameter
    }

    /// Returns a mutable reference to owned_parameter
    pub fn owned_parameter_mut(&mut self) -> &mut Vec<Parameter> {
        &mut self.owned_parameter
    }

    /// Adds an item to owned_parameter
    pub fn add_owned_parameter(&mut self, item: Parameter) {
        self.owned_parameter.push(item);
    }

    /// Clears all items from owned_parameter
    pub fn clear_owned_parameter(&mut self) {
        self.owned_parameter.clear();
    }

    /// Returns is_abstract
    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    /// Sets is_abstract
    pub fn set_is_abstract(&mut self, value: bool) {
        self.is_abstract = value;
    }

    /// Returns a reference to method
    pub fn method(&self) -> &Vec<Rc<RefCell<Behavior>>> {
        &self.method
    }

    /// Returns a mutable reference to method
    pub fn method_mut(&mut self) -> &mut Vec<Rc<RefCell<Behavior>>> {
        &mut self.method
    }

    /// Adds an item to method
    pub fn add_method(&mut self, item: Rc<RefCell<Behavior>>) {
        self.method.push(item);
    }

    /// Clears all items from method
    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    /// Returns concurrency as a string slice
    pub fn concurrency(&self) -> &str {
        &self.concurrency
    }

    /// Sets concurrency
    pub fn set_concurrency(&mut self, value: impl Into<String>) {
        self.concurrency = value.into();
    }

    /// Takes ownership of concurrency, replacing it with an empty string
    pub fn take_concurrency(&mut self) -> String {
        std::mem::take(&mut self.concurrency)
    }

    /// Returns a reference to raised_exception
    pub fn raised_exception(&self) -> &Vec<Rc<RefCell<Type>>> {
        &self.raised_exception
    }

    /// Returns a mutable reference to raised_exception
    pub fn raised_exception_mut(&mut self) -> &mut Vec<Rc<RefCell<Type>>> {
        &mut self.raised_exception
    }

    /// Adds an item to raised_exception
    pub fn add_raised_exception(&mut self, item: Rc<RefCell<Type>>) {
        self.raised_exception.push(item);
    }

    /// Clears all items from raised_exception
    pub fn clear_raised_exception(&mut self) {
        self.raised_exception.clear();
    }

    /// Returns a slice of owned_parameter_set
    pub fn owned_parameter_set(&self) -> &[ParameterSet] {
        &self.owned_parameter_set
    }

    /// Returns a mutable reference to owned_parameter_set
    pub fn owned_parameter_set_mut(&mut self) -> &mut Vec<ParameterSet> {
        &mut self.owned_parameter_set
    }

    /// Adds an item to owned_parameter_set
    pub fn add_owned_parameter_set(&mut self, item: ParameterSet) {
        self.owned_parameter_set.push(item);
    }

    /// Clears all items from owned_parameter_set
    pub fn clear_owned_parameter_set(&mut self) {
        self.owned_parameter_set.clear();
    }

}

