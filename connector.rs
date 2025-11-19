use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::association::Association;
use crate::connector_end::ConnectorEnd;
use crate::connector_kind::ConnectorKind;
use crate::behavior::Behavior;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Connector {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    is_static: bool,
    type_: Option<Rc<RefCell<Association>>>,
    redefined_connector: Vec<Rc<RefCell<Connector>>>,
    end: Vec<ConnectorEnd>,
    kind: Option<String>,
    contract: Vec<Rc<RefCell<Behavior>>>,
}

impl Connector {
    pub fn new(is_leaf: bool, is_static: bool, end: Vec<ConnectorEnd>) -> Self {
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
            redefined_connector: Vec::new(),
            end: end,
            kind: None,
            contract: Vec::new(),
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
    pub fn type_(&self) -> Option<&Rc<RefCell<Association>>> {
        self.type_.as_ref()
    }

    /// Returns a mutable reference to type_ if present
    pub fn type_mut(&mut self) -> Option<&mut Rc<RefCell<Association>>> {
        self.type_.as_mut()
    }

    /// Sets type_
    pub fn set_type_(&mut self, value: Rc<RefCell<Association>>) {
        self.type_ = Some(value);
    }

    /// Takes type_, leaving None in its place
    pub fn take_type(&mut self) -> Option<Rc<RefCell<Association>>> {
        self.type_.take()
    }

    /// Returns a reference to redefined_connector
    pub fn redefined_connector(&self) -> &Vec<Rc<RefCell<Connector>>> {
        &self.redefined_connector
    }

    /// Returns a mutable reference to redefined_connector
    pub fn redefined_connector_mut(&mut self) -> &mut Vec<Rc<RefCell<Connector>>> {
        &mut self.redefined_connector
    }

    /// Adds an item to redefined_connector
    pub fn add_redefined_connector(&mut self, item: Rc<RefCell<Connector>>) {
        self.redefined_connector.push(item);
    }

    /// Clears all items from redefined_connector
    pub fn clear_redefined_connector(&mut self) {
        self.redefined_connector.clear();
    }

    /// Returns a slice of end
    pub fn end(&self) -> &[ConnectorEnd] {
        &self.end
    }

    /// Returns a mutable reference to end
    pub fn end_mut(&mut self) -> &mut Vec<ConnectorEnd> {
        &mut self.end
    }

    /// Adds an item to end
    pub fn add_end(&mut self, item: ConnectorEnd) {
        self.end.push(item);
    }

    /// Clears all items from end
    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    /// Returns a reference to kind if present
    pub fn kind(&self) -> Option<&String> {
        self.kind.as_ref()
    }

    /// Returns a mutable reference to kind if present
    pub fn kind_mut(&mut self) -> Option<&mut String> {
        self.kind.as_mut()
    }

    /// Sets kind
    pub fn set_kind(&mut self, value: String) {
        self.kind = Some(value);
    }

    /// Takes kind, leaving None in its place
    pub fn take_kind(&mut self) -> Option<String> {
        self.kind.take()
    }

    /// Returns a reference to contract
    pub fn contract(&self) -> &Vec<Rc<RefCell<Behavior>>> {
        &self.contract
    }

    /// Returns a mutable reference to contract
    pub fn contract_mut(&mut self) -> &mut Vec<Rc<RefCell<Behavior>>> {
        &mut self.contract
    }

    /// Adds an item to contract
    pub fn add_contract(&mut self, item: Rc<RefCell<Behavior>>) {
        self.contract.push(item);
    }

    /// Clears all items from contract
    pub fn clear_contract(&mut self) {
        self.contract.clear();
    }

}

impl std::fmt::Display for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

