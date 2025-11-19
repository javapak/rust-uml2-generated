use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::constraint::Constraint;
use crate::template_parameter::TemplateParameter;
use crate::template_binding::TemplateBinding;
use crate::template_signature::TemplateSignature;
use crate::generalization::Generalization;
use crate::generalization_set::GeneralizationSet;
use crate::classifier::Classifier;
use crate::substitution::Substitution;
use crate::collaboration_use::CollaborationUse;
use crate::use_case::UseCase;
use crate::behavior::Behavior;
use crate::interface_realization::InterfaceRealization;
use crate::trigger::Trigger;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BehavioredClassifier {
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
    owning_template_parameter: Option<Weak<RefCell<TemplateParameter>>>,
    template_parameter: Option<Rc<RefCell<TemplateParameter>>>,
    template_binding: Vec<TemplateBinding>,
    owned_template_signature: Option<TemplateSignature>,
    is_abstract: bool,
    generalization: Vec<Generalization>,
    powertype_extent: Vec<Rc<RefCell<GeneralizationSet>>>,
    redefined_classifier: Vec<Rc<RefCell<Classifier>>>,
    substitution: Vec<Substitution>,
    representation: Option<Rc<RefCell<CollaborationUse>>>,
    collaboration_use: Vec<CollaborationUse>,
    owned_use_case: Vec<UseCase>,
    use_case: Vec<Rc<RefCell<UseCase>>>,
    owned_behavior: Vec<Behavior>,
    classifier_behavior: Option<Rc<RefCell<Behavior>>>,
    interface_realization: Vec<InterfaceRealization>,
    owned_trigger: Vec<Trigger>,
}

impl BehavioredClassifier {
    pub fn new(is_leaf: bool, is_abstract: bool) -> Self {
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
            owning_template_parameter: None,
            template_parameter: None,
            template_binding: Vec::new(),
            owned_template_signature: None,
            is_abstract: is_abstract,
            generalization: Vec::new(),
            powertype_extent: Vec::new(),
            redefined_classifier: Vec::new(),
            substitution: Vec::new(),
            representation: None,
            collaboration_use: Vec::new(),
            owned_use_case: Vec::new(),
            use_case: Vec::new(),
            owned_behavior: Vec::new(),
            classifier_behavior: None,
            interface_realization: Vec::new(),
            owned_trigger: Vec::new(),
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

    /// Returns is_abstract
    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    /// Sets is_abstract
    pub fn set_is_abstract(&mut self, value: bool) {
        self.is_abstract = value;
    }

    /// Returns a slice of generalization
    pub fn generalization(&self) -> &[Generalization] {
        &self.generalization
    }

    /// Returns a mutable reference to generalization
    pub fn generalization_mut(&mut self) -> &mut Vec<Generalization> {
        &mut self.generalization
    }

    /// Adds an item to generalization
    pub fn add_generalization(&mut self, item: Generalization) {
        self.generalization.push(item);
    }

    /// Clears all items from generalization
    pub fn clear_generalization(&mut self) {
        self.generalization.clear();
    }

    /// Returns a reference to powertype_extent
    pub fn powertype_extent(&self) -> &Vec<Rc<RefCell<GeneralizationSet>>> {
        &self.powertype_extent
    }

    /// Returns a mutable reference to powertype_extent
    pub fn powertype_extent_mut(&mut self) -> &mut Vec<Rc<RefCell<GeneralizationSet>>> {
        &mut self.powertype_extent
    }

    /// Adds an item to powertype_extent
    pub fn add_powertype_extent(&mut self, item: Rc<RefCell<GeneralizationSet>>) {
        self.powertype_extent.push(item);
    }

    /// Clears all items from powertype_extent
    pub fn clear_powertype_extent(&mut self) {
        self.powertype_extent.clear();
    }

    /// Returns a reference to redefined_classifier
    pub fn redefined_classifier(&self) -> &Vec<Rc<RefCell<Classifier>>> {
        &self.redefined_classifier
    }

    /// Returns a mutable reference to redefined_classifier
    pub fn redefined_classifier_mut(&mut self) -> &mut Vec<Rc<RefCell<Classifier>>> {
        &mut self.redefined_classifier
    }

    /// Adds an item to redefined_classifier
    pub fn add_redefined_classifier(&mut self, item: Rc<RefCell<Classifier>>) {
        self.redefined_classifier.push(item);
    }

    /// Clears all items from redefined_classifier
    pub fn clear_redefined_classifier(&mut self) {
        self.redefined_classifier.clear();
    }

    /// Returns a slice of substitution
    pub fn substitution(&self) -> &[Substitution] {
        &self.substitution
    }

    /// Returns a mutable reference to substitution
    pub fn substitution_mut(&mut self) -> &mut Vec<Substitution> {
        &mut self.substitution
    }

    /// Adds an item to substitution
    pub fn add_substitution(&mut self, item: Substitution) {
        self.substitution.push(item);
    }

    /// Clears all items from substitution
    pub fn clear_substitution(&mut self) {
        self.substitution.clear();
    }

    /// Returns a reference to representation if present
    pub fn representation(&self) -> Option<&Rc<RefCell<CollaborationUse>>> {
        self.representation.as_ref()
    }

    /// Returns a mutable reference to representation if present
    pub fn representation_mut(&mut self) -> Option<&mut Rc<RefCell<CollaborationUse>>> {
        self.representation.as_mut()
    }

    /// Sets representation
    pub fn set_representation(&mut self, value: Rc<RefCell<CollaborationUse>>) {
        self.representation = Some(value);
    }

    /// Takes representation, leaving None in its place
    pub fn take_representation(&mut self) -> Option<Rc<RefCell<CollaborationUse>>> {
        self.representation.take()
    }

    /// Returns a slice of collaboration_use
    pub fn collaboration_use(&self) -> &[CollaborationUse] {
        &self.collaboration_use
    }

    /// Returns a mutable reference to collaboration_use
    pub fn collaboration_use_mut(&mut self) -> &mut Vec<CollaborationUse> {
        &mut self.collaboration_use
    }

    /// Adds an item to collaboration_use
    pub fn add_collaboration_use(&mut self, item: CollaborationUse) {
        self.collaboration_use.push(item);
    }

    /// Clears all items from collaboration_use
    pub fn clear_collaboration_use(&mut self) {
        self.collaboration_use.clear();
    }

    /// Returns a slice of owned_use_case
    pub fn owned_use_case(&self) -> &[UseCase] {
        &self.owned_use_case
    }

    /// Returns a mutable reference to owned_use_case
    pub fn owned_use_case_mut(&mut self) -> &mut Vec<UseCase> {
        &mut self.owned_use_case
    }

    /// Adds an item to owned_use_case
    pub fn add_owned_use_case(&mut self, item: UseCase) {
        self.owned_use_case.push(item);
    }

    /// Clears all items from owned_use_case
    pub fn clear_owned_use_case(&mut self) {
        self.owned_use_case.clear();
    }

    /// Returns a reference to use_case
    pub fn use_case(&self) -> &Vec<Rc<RefCell<UseCase>>> {
        &self.use_case
    }

    /// Returns a mutable reference to use_case
    pub fn use_case_mut(&mut self) -> &mut Vec<Rc<RefCell<UseCase>>> {
        &mut self.use_case
    }

    /// Adds an item to use_case
    pub fn add_use_case(&mut self, item: Rc<RefCell<UseCase>>) {
        self.use_case.push(item);
    }

    /// Clears all items from use_case
    pub fn clear_use_case(&mut self) {
        self.use_case.clear();
    }

    /// Returns a slice of owned_behavior
    pub fn owned_behavior(&self) -> &[Behavior] {
        &self.owned_behavior
    }

    /// Returns a mutable reference to owned_behavior
    pub fn owned_behavior_mut(&mut self) -> &mut Vec<Behavior> {
        &mut self.owned_behavior
    }

    /// Adds an item to owned_behavior
    pub fn add_owned_behavior(&mut self, item: Behavior) {
        self.owned_behavior.push(item);
    }

    /// Clears all items from owned_behavior
    pub fn clear_owned_behavior(&mut self) {
        self.owned_behavior.clear();
    }

    /// Returns a reference to classifier_behavior if present
    pub fn classifier_behavior(&self) -> Option<&Rc<RefCell<Behavior>>> {
        self.classifier_behavior.as_ref()
    }

    /// Returns a mutable reference to classifier_behavior if present
    pub fn classifier_behavior_mut(&mut self) -> Option<&mut Rc<RefCell<Behavior>>> {
        self.classifier_behavior.as_mut()
    }

    /// Sets classifier_behavior
    pub fn set_classifier_behavior(&mut self, value: Rc<RefCell<Behavior>>) {
        self.classifier_behavior = Some(value);
    }

    /// Takes classifier_behavior, leaving None in its place
    pub fn take_classifier_behavior(&mut self) -> Option<Rc<RefCell<Behavior>>> {
        self.classifier_behavior.take()
    }

    /// Returns a slice of interface_realization
    pub fn interface_realization(&self) -> &[InterfaceRealization] {
        &self.interface_realization
    }

    /// Returns a mutable reference to interface_realization
    pub fn interface_realization_mut(&mut self) -> &mut Vec<InterfaceRealization> {
        &mut self.interface_realization
    }

    /// Adds an item to interface_realization
    pub fn add_interface_realization(&mut self, item: InterfaceRealization) {
        self.interface_realization.push(item);
    }

    /// Clears all items from interface_realization
    pub fn clear_interface_realization(&mut self) {
        self.interface_realization.clear();
    }

    /// Returns a slice of owned_trigger
    pub fn owned_trigger(&self) -> &[Trigger] {
        &self.owned_trigger
    }

    /// Returns a mutable reference to owned_trigger
    pub fn owned_trigger_mut(&mut self) -> &mut Vec<Trigger> {
        &mut self.owned_trigger
    }

    /// Adds an item to owned_trigger
    pub fn add_owned_trigger(&mut self, item: Trigger) {
        self.owned_trigger.push(item);
    }

    /// Clears all items from owned_trigger
    pub fn clear_owned_trigger(&mut self) {
        self.owned_trigger.clear();
    }

}

