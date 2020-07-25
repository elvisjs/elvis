//! Column Styles
use crate::value::{layouts::MultiColumnLineStyle, Colors, Unit};

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

impl ToString for MultiColumnStyle {
    fn to_string(&self) -> String {
        let mut ss = "".to_string();
        ss.push_str(&format!("column-count: {}", self.count.to_string()));
        ss.push_str(&format!("column-gap: {}", self.gap.to_string()));
        ss.push_str(&format!("column-rule-color: {}", self.color.to_string()));
        ss.push_str(&format!("column-rule-style: {}", self.style.to_string()));
        ss
    }
}