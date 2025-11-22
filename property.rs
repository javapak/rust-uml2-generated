// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Property (struct)
// Source Package: uml
// Package URI:    http://www.eclipse.org/uml2/2.1.0/UML
// Generated:      2025-11-22 12:14:06
// Generator:      EcoreToRustGenerator v0.1.0
//
// Generation Options:
//   - WASM:       enabled
//   - Tsify:      disabled
//   - Serde:      enabled
//   - Builders:   disabled
//   - References: String IDs
//
// WARNING: This file is auto-generated. Manual changes will be overwritten.
// ============================================================================

use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::string_expression::StringExpression;
use crate::value_specification::ValueSpecification;
use crate::deployment::Deployment;
use crate::template_binding::TemplateBinding;
use crate::template_signature::TemplateSignature;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct Property {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    is_static: bool,
    type_: Option<String>,
    is_ordered: bool,
    is_unique: bool,
    upper_value: Option<ValueSpecification>,
    lower_value: Option<ValueSpecification>,
    is_read_only: bool,
    owning_template_parameter: Option<String>,
    template_parameter: Option<String>,
    deployment: Vec<Deployment>,
    template_binding: Vec<TemplateBinding>,
    owned_template_signature: Option<TemplateSignature>,
    datatype: Option<String>,
    is_derived: bool,
    is_derived_union: bool,
    aggregation: String,
    redefined_property: Vec<String>,
    owning_association: Option<String>,
    default_value: Option<ValueSpecification>,
    subsetted_property: Vec<String>,
    association: Option<String>,
    qualifier: Vec<Box<Property>>,
    association_end: Option<String>,
}

#[wasm_bindgen]
impl Property {
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

    /// Returns a clone of name if present
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// Sets name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Takes name, leaving None in its place
    pub fn take_name(&mut self) -> Option<String> {
        self.name.take()
    }

    /// Returns a clone of visibility if present
    pub fn visibility(&self) -> Option<String> {
        self.visibility.clone()
    }

    /// Sets visibility
    pub fn set_visibility(&mut self, value: String) {
        self.visibility = Some(value);
    }

    /// Takes visibility, leaving None in its place
    pub fn take_visibility(&mut self) -> Option<String> {
        self.visibility.take()
    }

    /// Returns a clone of client_dependency
    pub fn client_dependency(&self) -> Vec<String> {
        self.client_dependency.clone()
    }

    /// Adds an existing Dependency to client_dependency by ID
    pub fn add_client_dependency_by_id(&mut self, id: String) {
        self.client_dependency.push(id);
    }

    /// Clears all items from client_dependency
    pub fn clear_client_dependency(&mut self) {
        self.client_dependency.clear();
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

    /// Returns a clone of type_ if present
    pub fn type_(&self) -> Option<String> {
        self.type_.clone()
    }

    /// Sets type_
    pub fn set_type_(&mut self, value: String) {
        self.type_ = Some(value);
    }

    /// Takes type_, leaving None in its place
    pub fn take_type(&mut self) -> Option<String> {
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

    /// Returns is_read_only
    pub fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    /// Sets is_read_only
    pub fn set_is_read_only(&mut self, value: bool) {
        self.is_read_only = value;
    }

    /// Returns a clone of owning_template_parameter if present
    pub fn owning_template_parameter(&self) -> Option<String> {
        self.owning_template_parameter.clone()
    }

    /// Sets owning_template_parameter
    pub fn set_owning_template_parameter(&mut self, value: String) {
        self.owning_template_parameter = Some(value);
    }

    /// Takes owning_template_parameter, leaving None in its place
    pub fn take_owning_template_parameter(&mut self) -> Option<String> {
        self.owning_template_parameter.take()
    }

    /// Returns a clone of template_parameter if present
    pub fn template_parameter(&self) -> Option<String> {
        self.template_parameter.clone()
    }

    /// Sets template_parameter
    pub fn set_template_parameter(&mut self, value: String) {
        self.template_parameter = Some(value);
    }

    /// Takes template_parameter, leaving None in its place
    pub fn take_template_parameter(&mut self) -> Option<String> {
        self.template_parameter.take()
    }

    /// Returns a clone of datatype if present
    pub fn datatype(&self) -> Option<String> {
        self.datatype.clone()
    }

    /// Sets datatype
    pub fn set_datatype(&mut self, value: String) {
        self.datatype = Some(value);
    }

    /// Takes datatype, leaving None in its place
    pub fn take_datatype(&mut self) -> Option<String> {
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

    /// Returns a clone of aggregation
    pub fn aggregation(&self) -> String {
        self.aggregation.clone()
    }

    /// Sets aggregation
    pub fn set_aggregation(&mut self, value: String) {
        self.aggregation = value;
    }

    /// Takes ownership of aggregation, replacing it with an empty string
    pub fn take_aggregation(&mut self) -> String {
        std::mem::take(&mut self.aggregation)
    }

    /// Returns a clone of redefined_property
    pub fn redefined_property(&self) -> Vec<String> {
        self.redefined_property.clone()
    }

    /// Adds an existing Property to redefined_property by ID
    pub fn add_redefined_property_by_id(&mut self, id: String) {
        self.redefined_property.push(id);
    }

    /// Clears all items from redefined_property
    pub fn clear_redefined_property(&mut self) {
        self.redefined_property.clear();
    }

    /// Returns a clone of owning_association if present
    pub fn owning_association(&self) -> Option<String> {
        self.owning_association.clone()
    }

    /// Sets owning_association
    pub fn set_owning_association(&mut self, value: String) {
        self.owning_association = Some(value);
    }

    /// Takes owning_association, leaving None in its place
    pub fn take_owning_association(&mut self) -> Option<String> {
        self.owning_association.take()
    }

    /// Returns a clone of subsetted_property
    pub fn subsetted_property(&self) -> Vec<String> {
        self.subsetted_property.clone()
    }

    /// Adds an existing Property to subsetted_property by ID
    pub fn add_subsetted_property_by_id(&mut self, id: String) {
        self.subsetted_property.push(id);
    }

    /// Clears all items from subsetted_property
    pub fn clear_subsetted_property(&mut self) {
        self.subsetted_property.clear();
    }

    /// Returns a clone of association if present
    pub fn association(&self) -> Option<String> {
        self.association.clone()
    }

    /// Sets association
    pub fn set_association(&mut self, value: String) {
        self.association = Some(value);
    }

    /// Takes association, leaving None in its place
    pub fn take_association(&mut self) -> Option<String> {
        self.association.take()
    }

    /// Returns a clone of association_end if present
    pub fn association_end(&self) -> Option<String> {
        self.association_end.clone()
    }

    /// Sets association_end
    pub fn set_association_end(&mut self, value: String) {
        self.association_end = Some(value);
    }

    /// Takes association_end, leaving None in its place
    pub fn take_association_end(&mut self) -> Option<String> {
        self.association_end.take()
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|e| e.to_string())
    }

    /// Deserialize from JSON string
    pub fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json)
            .map_err(|e| e.to_string())
    }

    /// Returns whether this type can be created standalone (not nested)
    pub fn can_exist_standalone() -> bool {
        true
    }

    /// Returns whether this type requires a container
    pub fn requires_container() -> bool {
        false
    }

    /// Returns the type name
    pub fn type_name() -> String {
        "Property".to_string()
    }

}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

