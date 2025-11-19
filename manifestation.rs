use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::template_parameter::TemplateParameter;
use crate::named_element::NamedElement;
use crate::opaque_expression::OpaqueExpression;
use crate::packageable_element::PackageableElement;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifestation {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    owning_template_parameter: Option<Weak<RefCell<TemplateParameter>>>,
    template_parameter: Option<Rc<RefCell<TemplateParameter>>>,
    supplier: Vec<Rc<RefCell<NamedElement>>>,
    client: Vec<Rc<RefCell<NamedElement>>>,
    mapping: Option<OpaqueExpression>,
    utilized_element: Rc<RefCell<PackageableElement>>,
}

impl Manifestation {
    pub fn new(supplier: Vec<Rc<RefCell<NamedElement>>>, client: Vec<Rc<RefCell<NamedElement>>>, utilized_element: Rc<RefCell<PackageableElement>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            owning_template_parameter: None,
            template_parameter: None,
            supplier: supplier,
            client: client,
            mapping: None,
            utilized_element: utilized_element,
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

    /// Returns a reference to owning_template_parameter if present
    pub fn owning_template_parameter(&self) -> Option<&Weak<RefCell<TemplateParameter>>> {
        self.owning_template_parameter.as_ref()
    }

    /// Returns a mutable reference to owning_template_parameter if present
    pub fn owning_template_parameter_mut(&mut self) -> Option<&mut Weak<RefCell<TemplateParameter>>> {
        self.owning_template_parameter.as_mut()
    }

    /// Sets owning_template_parameter
    pub fn set_owning_template_parameter(&mut self, value: Weak<RefCell<TemplateParameter>>) {
        self.owning_template_parameter = Some(value);
    }

    /// Takes owning_template_parameter, leaving None in its place
    pub fn take_owning_template_parameter(&mut self) -> Option<Weak<RefCell<TemplateParameter>>> {
        self.owning_template_parameter.take()
    }

    /// Returns a reference to template_parameter if present
    pub fn template_parameter(&self) -> Option<&Rc<RefCell<TemplateParameter>>> {
        self.template_parameter.as_ref()
    }

    /// Returns a mutable reference to template_parameter if present
    pub fn template_parameter_mut(&mut self) -> Option<&mut Rc<RefCell<TemplateParameter>>> {
        self.template_parameter.as_mut()
    }

    /// Sets template_parameter
    pub fn set_template_parameter(&mut self, value: Rc<RefCell<TemplateParameter>>) {
        self.template_parameter = Some(value);
    }

    /// Takes template_parameter, leaving None in its place
    pub fn take_template_parameter(&mut self) -> Option<Rc<RefCell<TemplateParameter>>> {
        self.template_parameter.take()
    }

    /// Returns a reference to supplier
    pub fn supplier(&self) -> &Vec<Rc<RefCell<NamedElement>>> {
        &self.supplier
    }

    /// Returns a mutable reference to supplier
    pub fn supplier_mut(&mut self) -> &mut Vec<Rc<RefCell<NamedElement>>> {
        &mut self.supplier
    }

    /// Adds an item to supplier
    pub fn add_supplier(&mut self, item: Rc<RefCell<NamedElement>>) {
        self.supplier.push(item);
    }

    /// Clears all items from supplier
    pub fn clear_supplier(&mut self) {
        self.supplier.clear();
    }

    /// Returns a reference to client
    pub fn client(&self) -> &Vec<Rc<RefCell<NamedElement>>> {
        &self.client
    }

    /// Returns a mutable reference to client
    pub fn client_mut(&mut self) -> &mut Vec<Rc<RefCell<NamedElement>>> {
        &mut self.client
    }

    /// Adds an item to client
    pub fn add_client(&mut self, item: Rc<RefCell<NamedElement>>) {
        self.client.push(item);
    }

    /// Clears all items from client
    pub fn clear_client(&mut self) {
        self.client.clear();
    }

    /// Returns a reference to mapping if present
    pub fn mapping(&self) -> Option<&OpaqueExpression> {
        self.mapping.as_ref()
    }

    /// Returns a mutable reference to mapping if present
    pub fn mapping_mut(&mut self) -> Option<&mut OpaqueExpression> {
        self.mapping.as_mut()
    }

    /// Sets mapping
    pub fn set_mapping(&mut self, value: OpaqueExpression) {
        self.mapping = Some(value);
    }

    /// Takes mapping, leaving None in its place
    pub fn take_mapping(&mut self) -> Option<OpaqueExpression> {
        self.mapping.take()
    }

    /// Returns a reference to utilized_element
    pub fn utilized_element(&self) -> &Rc<RefCell<PackageableElement>> {
        &self.utilized_element
    }

    /// Returns a mutable reference to utilized_element
    pub fn utilized_element_mut(&mut self) -> &mut Rc<RefCell<PackageableElement>> {
        &mut self.utilized_element
    }

    /// Sets utilized_element
    pub fn set_utilized_element(&mut self, value: Rc<RefCell<PackageableElement>>) {
        self.utilized_element = value;
    }

}

impl Default for Manifestation {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            owning_template_parameter: None,
            template_parameter: None,
            supplier: Vec::new(),
            client: Vec::new(),
            mapping: None,
            utilized_element: Default::default(),
        }
    }
}

impl std::fmt::Display for Manifestation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

