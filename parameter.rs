use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::type_::Type;
use crate::template_parameter::TemplateParameter;
use crate::value_specification::ValueSpecification;
use crate::parameter_set::ParameterSet;
use crate::parameter_direction_kind::ParameterDirectionKind;
use crate::parameter_effect_kind::ParameterEffectKind;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Parameter {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    type_: Option<Rc<RefCell<Type>>>,
    owning_template_parameter: Option<Weak<RefCell<TemplateParameter>>>,
    template_parameter: Option<Rc<RefCell<TemplateParameter>>>,
    is_ordered: bool,
    is_unique: bool,
    upper_value: Option<ValueSpecification>,
    lower_value: Option<ValueSpecification>,
    parameter_set: Vec<Rc<RefCell<ParameterSet>>>,
    direction: String,
    default_value: Option<ValueSpecification>,
    is_exception: bool,
    is_stream: bool,
    effect: Option<String>,
}

impl Parameter {
    pub fn new(is_ordered: bool, is_unique: bool, direction: String, is_exception: bool, is_stream: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            type_: None,
            owning_template_parameter: None,
            template_parameter: None,
            is_ordered: is_ordered,
            is_unique: is_unique,
            upper_value: None,
            lower_value: None,
            parameter_set: Vec::new(),
            direction: direction,
            default_value: None,
            is_exception: is_exception,
            is_stream: is_stream,
            effect: None,
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

    /// Returns a reference to parameter_set
    pub fn parameter_set(&self) -> &Vec<Rc<RefCell<ParameterSet>>> {
        &self.parameter_set
    }

    /// Returns a mutable reference to parameter_set
    pub fn parameter_set_mut(&mut self) -> &mut Vec<Rc<RefCell<ParameterSet>>> {
        &mut self.parameter_set
    }

    /// Adds an item to parameter_set
    pub fn add_parameter_set(&mut self, item: Rc<RefCell<ParameterSet>>) {
        self.parameter_set.push(item);
    }

    /// Clears all items from parameter_set
    pub fn clear_parameter_set(&mut self) {
        self.parameter_set.clear();
    }

    /// Returns direction as a string slice
    pub fn direction(&self) -> &str {
        &self.direction
    }

    /// Sets direction
    pub fn set_direction(&mut self, value: impl Into<String>) {
        self.direction = value.into();
    }

    /// Takes ownership of direction, replacing it with an empty string
    pub fn take_direction(&mut self) -> String {
        std::mem::take(&mut self.direction)
    }

    /// Returns a reference to default_value if present
    pub fn default_value(&self) -> Option<&ValueSpecification> {
        self.default_value.as_ref()
    }

    /// Returns a mutable reference to default_value if present
    pub fn default_value_mut(&mut self) -> Option<&mut ValueSpecification> {
        self.default_value.as_mut()
    }

    /// Sets default_value
    pub fn set_default_value(&mut self, value: ValueSpecification) {
        self.default_value = Some(value);
    }

    /// Takes default_value, leaving None in its place
    pub fn take_default_value(&mut self) -> Option<ValueSpecification> {
        self.default_value.take()
    }

    /// Returns is_exception
    pub fn is_exception(&self) -> bool {
        self.is_exception
    }

    /// Sets is_exception
    pub fn set_is_exception(&mut self, value: bool) {
        self.is_exception = value;
    }

    /// Returns is_stream
    pub fn is_stream(&self) -> bool {
        self.is_stream
    }

    /// Sets is_stream
    pub fn set_is_stream(&mut self, value: bool) {
        self.is_stream = value;
    }

    /// Returns a reference to effect if present
    pub fn effect(&self) -> Option<&String> {
        self.effect.as_ref()
    }

    /// Returns a mutable reference to effect if present
    pub fn effect_mut(&mut self) -> Option<&mut String> {
        self.effect.as_mut()
    }

    /// Sets effect
    pub fn set_effect(&mut self, value: String) {
        self.effect = Some(value);
    }

    /// Takes effect, leaving None in its place
    pub fn take_effect(&mut self) -> Option<String> {
        self.effect.take()
    }

}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

