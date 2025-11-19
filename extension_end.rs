use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::type_::Type;
use crate::value_specification::ValueSpecification;
use crate::template_parameter::TemplateParameter;
use crate::deployment::Deployment;
use crate::template_binding::TemplateBinding;
use crate::template_signature::TemplateSignature;
use crate::data_type::DataType;
use crate::aggregation_kind::AggregationKind;
use crate::property::Property;
use crate::association::Association;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionEnd {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    is_static: bool,
    type_: Option<Rc<RefCell<Type>>>,
    is_ordered: bool,
    is_unique: bool,
    upper_value: Option<ValueSpecification>,
    lower_value: Option<ValueSpecification>,
    is_read_only: bool,
    owning_template_parameter: Option<Weak<RefCell<TemplateParameter>>>,
    template_parameter: Option<Rc<RefCell<TemplateParameter>>>,
    deployment: Vec<Deployment>,
    template_binding: Vec<TemplateBinding>,
    owned_template_signature: Option<TemplateSignature>,
    datatype: Option<Weak<RefCell<DataType>>>,
    is_derived: bool,
    is_derived_union: bool,
    aggregation: String,
    redefined_property: Vec<Rc<RefCell<Property>>>,
    owning_association: Option<Weak<RefCell<Association>>>,
    default_value: Option<ValueSpecification>,
    subsetted_property: Vec<Rc<RefCell<Property>>>,
    association: Option<Rc<RefCell<Association>>>,
    qualifier: Vec<Property>,
    association_end: Option<Weak<RefCell<Property>>>,
}

impl ExtensionEnd {
    pub fn new(is_leaf: bool, is_static: bool, is_ordered: bool, is_unique: bool, is_read_only: bool, is_derived: bool, is_derived_union: bool, aggregation: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            is_static: is_static,
            type_: None,
            is_ordered: is_ordered,
            is_unique: is_unique,
            upper_value: None,
            lower_value: None,
            is_read_only: is_read_only,
            owning_template_parameter: None,
            template_parameter: None,
            deployment: Vec::new(),
            template_binding: Vec::new(),
            owned_template_signature: None,
            datatype: None,
            is_derived: is_derived,
            is_derived_union: is_derived_union,
            aggregation: aggregation,
            redefined_property: Vec::new(),
            owning_association: None,
            default_value: None,
            subsetted_property: Vec::new(),
            association: None,
            qualifier: Vec::new(),
            association_end: None,
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

    /// Returns is_static
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    /// Sets is_static
    pub fn set_is_static(&mut self, value: bool) {
        self.is_static = value;
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

    /// Returns is_read_only
    pub fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    /// Sets is_read_only
    pub fn set_is_read_only(&mut self, value: bool) {
        self.is_read_only = value;
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

    /// Returns a slice of deployment
    pub fn deployment(&self) -> &[Deployment] {
        &self.deployment
    }

    /// Returns a mutable reference to deployment
    pub fn deployment_mut(&mut self) -> &mut Vec<Deployment> {
        &mut self.deployment
    }

    /// Adds an item to deployment
    pub fn add_deployment(&mut self, item: Deployment) {
        self.deployment.push(item);
    }

    /// Clears all items from deployment
    pub fn clear_deployment(&mut self) {
        self.deployment.clear();
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

    /// Returns is_derived
    pub fn is_derived(&self) -> bool {
        self.is_derived
    }

    /// Sets is_derived
    pub fn set_is_derived(&mut self, value: bool) {
        self.is_derived = value;
    }

    /// Returns is_derived_union
    pub fn is_derived_union(&self) -> bool {
        self.is_derived_union
    }

    /// Sets is_derived_union
    pub fn set_is_derived_union(&mut self, value: bool) {
        self.is_derived_union = value;
    }

    /// Returns aggregation as a string slice
    pub fn aggregation(&self) -> &str {
        &self.aggregation
    }

    /// Sets aggregation
    pub fn set_aggregation(&mut self, value: impl Into<String>) {
        self.aggregation = value.into();
    }

    /// Takes ownership of aggregation, replacing it with an empty string
    pub fn take_aggregation(&mut self) -> String {
        std::mem::take(&mut self.aggregation)
    }

    /// Returns a reference to redefined_property
    pub fn redefined_property(&self) -> &Vec<Rc<RefCell<Property>>> {
        &self.redefined_property
    }

    /// Returns a mutable reference to redefined_property
    pub fn redefined_property_mut(&mut self) -> &mut Vec<Rc<RefCell<Property>>> {
        &mut self.redefined_property
    }

    /// Adds an item to redefined_property
    pub fn add_redefined_property(&mut self, item: Rc<RefCell<Property>>) {
        self.redefined_property.push(item);
    }

    /// Clears all items from redefined_property
    pub fn clear_redefined_property(&mut self) {
        self.redefined_property.clear();
    }

    /// Returns a reference to owning_association if present
    pub fn owning_association(&self) -> Option<&Weak<RefCell<Association>>> {
        self.owning_association.as_ref()
    }

    /// Returns a mutable reference to owning_association if present
    pub fn owning_association_mut(&mut self) -> Option<&mut Weak<RefCell<Association>>> {
        self.owning_association.as_mut()
    }

    /// Sets owning_association
    pub fn set_owning_association(&mut self, value: Weak<RefCell<Association>>) {
        self.owning_association = Some(value);
    }

    /// Takes owning_association, leaving None in its place
    pub fn take_owning_association(&mut self) -> Option<Weak<RefCell<Association>>> {
        self.owning_association.take()
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

    /// Returns a reference to subsetted_property
    pub fn subsetted_property(&self) -> &Vec<Rc<RefCell<Property>>> {
        &self.subsetted_property
    }

    /// Returns a mutable reference to subsetted_property
    pub fn subsetted_property_mut(&mut self) -> &mut Vec<Rc<RefCell<Property>>> {
        &mut self.subsetted_property
    }

    /// Adds an item to subsetted_property
    pub fn add_subsetted_property(&mut self, item: Rc<RefCell<Property>>) {
        self.subsetted_property.push(item);
    }

    /// Clears all items from subsetted_property
    pub fn clear_subsetted_property(&mut self) {
        self.subsetted_property.clear();
    }

    /// Returns a reference to association if present
    pub fn association(&self) -> Option<&Rc<RefCell<Association>>> {
        self.association.as_ref()
    }

    /// Returns a mutable reference to association if present
    pub fn association_mut(&mut self) -> Option<&mut Rc<RefCell<Association>>> {
        self.association.as_mut()
    }

    /// Sets association
    pub fn set_association(&mut self, value: Rc<RefCell<Association>>) {
        self.association = Some(value);
    }

    /// Takes association, leaving None in its place
    pub fn take_association(&mut self) -> Option<Rc<RefCell<Association>>> {
        self.association.take()
    }

    /// Returns a slice of qualifier
    pub fn qualifier(&self) -> &[Property] {
        &self.qualifier
    }

    /// Returns a mutable reference to qualifier
    pub fn qualifier_mut(&mut self) -> &mut Vec<Property> {
        &mut self.qualifier
    }

    /// Adds an item to qualifier
    pub fn add_qualifier(&mut self, item: Property) {
        self.qualifier.push(item);
    }

    /// Clears all items from qualifier
    pub fn clear_qualifier(&mut self) {
        self.qualifier.clear();
    }

    /// Returns a reference to association_end if present
    pub fn association_end(&self) -> Option<&Weak<RefCell<Property>>> {
        self.association_end.as_ref()
    }

    /// Returns a mutable reference to association_end if present
    pub fn association_end_mut(&mut self) -> Option<&mut Weak<RefCell<Property>>> {
        self.association_end.as_mut()
    }

    /// Sets association_end
    pub fn set_association_end(&mut self, value: Weak<RefCell<Property>>) {
        self.association_end = Some(value);
    }

    /// Takes association_end, leaving None in its place
    pub fn take_association_end(&mut self) -> Option<Weak<RefCell<Property>>> {
        self.association_end.take()
    }

}

impl std::fmt::Display for ExtensionEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

