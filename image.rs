use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Image {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    content: Option<String>,
    location: Option<String>,
    format: Option<String>,
}

impl Image {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            content: None,
            location: None,
            format: None,
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

    /// Returns a reference to content if present
    pub fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }

    /// Returns a mutable reference to content if present
    pub fn content_mut(&mut self) -> Option<&mut String> {
        self.content.as_mut()
    }

    /// Sets content
    pub fn set_content(&mut self, value: String) {
        self.content = Some(value);
    }

    /// Takes content, leaving None in its place
    pub fn take_content(&mut self) -> Option<String> {
        self.content.take()
    }

    /// Returns a reference to location if present
    pub fn location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Returns a mutable reference to location if present
    pub fn location_mut(&mut self) -> Option<&mut String> {
        self.location.as_mut()
    }

    /// Sets location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Takes location, leaving None in its place
    pub fn take_location(&mut self) -> Option<String> {
        self.location.take()
    }

    /// Returns a reference to format if present
    pub fn format(&self) -> Option<&String> {
        self.format.as_ref()
    }

    /// Returns a mutable reference to format if present
    pub fn format_mut(&mut self) -> Option<&mut String> {
        self.format.as_mut()
    }

    /// Sets format
    pub fn set_format(&mut self, value: String) {
        self.format = Some(value);
    }

    /// Takes format, leaving None in its place
    pub fn take_format(&mut self) -> Option<String> {
        self.format.take()
    }

}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image(...)")
    }
}

