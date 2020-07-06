//! Elvis column layout
use crate::{
    widgets::values::{layouts::MultiColumnLineStyle, Colors, Unit},
    Node,
};

/// **Homework**: code a New York Times.
pub struct MultiColumn<T>
where
    T: Into<Node>,
{
    /// Column children
    pub children: Vec<T>,
    /// Column style
    pub style: MultiColumnStyle,
}

/// `Multicolumn` Style
pub struct MultiColumnStyle {
    /// Column color
    pub color: Colors,
    /// Column counts
    pub count: Unit,
    /// Column gap
    pub gap: Unit,
    /// Column line style
    pub style: MultiColumnLineStyle,
}
