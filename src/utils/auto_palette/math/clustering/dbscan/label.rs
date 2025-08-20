/// Struct representing a label for DBSCAN clustering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    Assigned(usize),
    Outlier,
    Marked,
    Undefined,
}

impl Label {
    /// Returns whether this label is assigned.
    ///
    /// # Returns
    /// `true` if the label is assigned, otherwise `false`.
    pub fn is_assigned(&self) -> bool {
        matches!(*self, Label::Assigned(_))
    }

    /// Returns whether this label is outlier.
    ///
    /// # Returns
    /// `true` if the label is outlier, otherwise `false`.
    pub fn is_outlier(&self) -> bool {
        matches!(*self, Label::Outlier)
    }

    /// Returns whether this label is undefined.
    ///
    /// # Returns
    /// `true` if the label is undefined, otherwise `false`.
    pub fn is_undefined(&self) -> bool {
        matches!(*self, Label::Undefined)
    }
}
