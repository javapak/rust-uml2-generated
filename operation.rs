use std::rc::Weak;
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
use crate::template_parameter::TemplateParameter;
use crate::template_binding::TemplateBinding;
use crate::template_signature::TemplateSignature;
use crate::interface::Interface;
use crate::class::Class;
use crate::data_type::DataType;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Operation {
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
    owning_template_parameter: Option<Weak<RefCell<TemplateParameter>>>,
    template_parameter: Option<Rc<RefCell<TemplateParameter>>>,
    template_binding: Vec<TemplateBinding>,
    owned_template_signature: Option<TemplateSignature>,
    interface: Option<Weak<RefCell<Interface>>>,
    class: Option<Weak<RefCell<Class>>>,
    is_query: bool,
    precondition: Vec<Rc<RefCell<Constraint>>>,
    postcondition: Vec<Rc<RefCell<Constraint>>>,
    redefined_operation: Vec<Rc<RefCell<Operation>>>,
    datatype: Option<Weak<RefCell<DataType>>>,
    body_condition: Option<Rc<RefCell<Constraint>>>,
}

impl Operation {
    pub fn new(is_leaf: bool, is_static: bool, is_abstract: bool, concurrency: String, is_query: bool) -> Self {
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
            owning_template_parameter: None,
            template_parameter: None,
            template_binding: Vec::new(),
            owned_template_signature: None,
            interface: None,
            class: None,
            is_query: is_query,
            precondition: Vec::new(),
            postcondition: Vec::new(),
            redefined_operation: Vec::new(),
            datatype: None,
            body_condition: None,
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

    /// Returns a slice of template_binding
    pub fn template_binding(&self) -> &[TemplateBinding] {
        &self.template_binding
    }

    /// Returns a mutable reference to template_binding
    pub fn template_binding_mut(&mut self) -> &mut Vec<TemplateBinding> {
        &mut self.template_binding
    }

    /// Adds an item to template_binding
    pub fn add_template_binding(&mut self, item: TemplateBinding) {
        self.template_binding.push(item);
    }

    /// Clears all items from template_binding
    pub fn clear_template_binding(&mut self) {
        self.template_binding.clear();
    }

    /// Returns a reference to owned_template_signature if present
    pub fn owned_template_signature(&self) -> Option<&TemplateSignature> {
        self.owned_template_signature.as_ref()
    }

    /// Returns a mutable reference to owned_template_signature if present
    pub fn owned_template_signature_mut(&mut self) -> Option<&mut TemplateSignature> {
        self.owned_template_signature.as_mut()
    }

    /// Sets owned_template_signature
    pub fn set_owned_template_signature(&mut self, value: TemplateSignature) {
        self.owned_template_signature = Some(value);
    }

    /// Takes owned_template_signature, leaving None in its place
    pub fn take_owned_template_signature(&mut self) -> Option<TemplateSignature> {
        self.owned_template_signature.take()
    }

    /// Returns a reference to interface if present
    pub fn interface(&self) -> Option<&Weak<RefCell<Interface>>> {
        self.interface.as_ref()
    }

    /// Returns a mutable reference to interface if present
    pub fn interface_mut(&mut self) -> Option<&mut Weak<RefCell<Interface>>> {
        self.interface.as_mut()
    }

    /// Sets interface
    pub fn set_interface(&mut self, value: Weak<RefCell<Interface>>) {
        self.interface = Some(value);
    }

    /// Takes interface, leaving None in its place
    pub fn take_interface(&mut self) -> Option<Weak<RefCell<Interface>>> {
        self.interface.take()
    }

    /// Returns a reference to class if present
    pub fn class(&self) -> Option<&Weak<RefCell<Class>>> {
        self.class.as_ref()
    }

    /// Returns a mutable reference to class if present
    pub fn class_mut(&mut self) -> Option<&mut Weak<RefCell<Class>>> {
        self.class.as_mut()
    }

    /// Sets class
    pub fn set_class(&mut self, value: Weak<RefCell<Class>>) {
        self.class = Some(value);
    }

    /// Takes class, leaving None in its place
    pub fn take_class(&mut self) -> Option<Weak<RefCell<Class>>> {
        self.class.take()
    }

    /// Returns is_query
    pub fn is_query(&self) -> bool {
        self.is_query
    }

    /// Sets is_query
    pub fn set_is_query(&mut self, value: bool) {
        self.is_query = value;
    }

    /// Returns a reference to precondition
    pub fn precondition(&self) -> &Vec<Rc<RefCell<Constraint>>> {
        &self.precondition
    }

    /// Returns a mutable reference to precondition
    pub fn precondition_mut(&mut self) -> &mut Vec<Rc<RefCell<Constraint>>> {
        &mut self.precondition
    }

    /// Adds an item to precondition
    pub fn add_precondition(&mut self, item: Rc<RefCell<Constraint>>) {
        self.precondition.push(item);
    }

    /// Clears all items from precondition
    pub fn clear_precondition(&mut self) {
        self.precondition.clear();
    }

    /// Returns a reference to postcondition
    pub fn postcondition(&self) -> &Vec<Rc<RefCell<Constraint>>> {
        &self.postcondition
    }

    /// Returns a mutable reference to postcondition
    pub fn postcondition_mut(&mut self) -> &mut Vec<Rc<RefCell<Constraint>>> {
        &mut self.postcondition
    }

    /// Adds an item to postcondition
    pub fn add_postcondition(&mut self, item: Rc<RefCell<Constraint>>) {
        self.postcondition.push(item);
    }

    /// Clears all items from postcondition
    pub fn clear_postcondition(&mut self) {
        self.postcondition.clear();
    }

    /// Returns a reference to redefined_operation
    pub fn redefined_operation(&self) -> &Vec<Rc<RefCell<Operation>>> {
        &self.redefined_operation
    }

    /// Returns a mutable reference to redefined_operation
    pub fn redefined_operation_mut(&mut self) -> &mut Vec<Rc<RefCell<Operation>>> {
        &mut self.redefined_operation
    }

    /// Adds an item to redefined_operation
    pub fn add_redefined_operation(&mut self, item: Rc<RefCell<Operation>>) {
        self.redefined_operation.push(item);
    }

    /// Clears all items from redefined_operation
    pub fn clear_redefined_operation(&mut self) {
        self.redefined_operation.clear();
    }

    /// Returns a reference to datatype if present
    pub fn datatype(&self) -> Option<&Weak<RefCell<DataType>>> {
        self.datatype.as_ref()
    }

    /// Returns a mutable reference to datatype if present
    pub fn datatype_mut(&mut self) -> Option<&mut Weak<RefCell<DataType>>> {
        self.datatype.as_mut()
    }

    /// Sets datatype
    pub fn set_datatype(&mut self, value: Weak<RefCell<DataType>>) {
        self.datatype = Some(value);
    }

    /// Takes datatype, leaving None in its place
    pub fn take_datatype(&mut self) -> Option<Weak<RefCell<DataType>>> {
        self.datatype.take()
    }

    /// Returns a reference to body_condition if present
    pub fn body_condition(&self) -> Option<&Rc<RefCell<Constraint>>> {
        self.body_condition.as_ref()
    }

    /// Returns a mutable reference to body_condition if present
    pub fn body_condition_mut(&mut self) -> Option<&mut Rc<RefCell<Constraint>>> {
        self.body_condition.as_mut()
    }

    /// Sets body_condition
    pub fn set_body_condition(&mut self, value: Rc<RefCell<Constraint>>) {
        self.body_condition = Some(value);
    }

    /// Takes body_condition, leaving None in its place
    pub fn take_body_condition(&mut self) -> Option<Rc<RefCell<Constraint>>> {
        self.body_condition.take()
    }

}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

