//! Widget Styles
use crate::{
    style::Style,
    value::{Color, FontFamily, FontStyle, Unit},
};
use elvis_core_support::Setter;

/// style of `Text`
#[derive(Clone, Debug, Eq, PartialEq, Setter)]
pub struct TextStyle {
    /// Bold text
    pub bold: bool,
    /// The color of the text
    pub color: Color,
    /// Italic text
    pub italic: bool,
    /// Text size
    pub size: Unit,
    /// Text weight
    pub weight: Unit,
    /// Text height
    pub height: Unit,
    /// Text stretch
    pub stretch: Unit,
    /// Font Family
    pub family: FontFamily,
}

impl Default for TextStyle {
    fn default() -> TextStyle {
        TextStyle {
            bold: false,
            color: Color::Pink,
            italic: false,
            size: Unit::Rem(2.0),
            weight: Unit::None(400.0),
            height: Unit::Rem(1.0),
            stretch: Unit::Percent(100.0),
            family: FontFamily::Helvetica,
        }
    }
}

impl Into<Vec<Style>> for TextStyle {
    fn into(self) -> Vec<Style> {
        vec![
            Style::Color(self.color),
            if self.bold {
                Style::FontWeight(Unit::None(700.0))
            } else {
                Style::FontWeight(self.weight)
            },
            if self.italic {
                Style::FontStyle(FontStyle::Italic)
            } else {
                Style::FontStyle(FontStyle::Normal)
            },
            Style::FontSize(self.size),
            Style::FontStretch(self.stretch),
            Style::LineHeight(self.height),
            Style::FontFamily(self.family),
        ]
    }
}

/// Image source
pub struct ImageSrc(pub String);

impl ImageSrc {
    /// Serialize source value as bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
