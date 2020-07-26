use crate::value::{
    layouts::{
        FlexBasis, FlexDirection, FlexPosition, GridAuto, GridFlow, GridTemplate,
        MultiColumnLineStyle,
    },
    Colors, Unit,
};

fn camel_snake(camel: &str) -> String {
    camel
        .split(|c: char| c.is_ascii_uppercase())
        .collect::<Vec<&str>>()
        .join("_")
}

macro_rules! construct_style {
    ($(($style:ident, $ty:ty, $doc:expr),)*) => {
        /// Evlis Style
        #[derive(Clone)]
        pub enum Style {
            $(
                #[doc=$doc]
                $style($ty),
            )*
        }

        impl ToString for Style {
            fn to_string(&self) -> String {
                match self {
                    $(
                        Style::$style(v) => format!(
                            "{}: {}",
                            camel_snake(stringify!($style)),
                            v.to_string()
                        ),
                     )*
                }
            }
        }
    };
    (
        [$(($ns:ident, $nty:ty, $ndoc:expr),)*],
        [$(($ds:ident, $dty:ty, $ddoc:expr),)*]
    ) => {
        construct_style!{
            $(($ns, $nty, $ndoc),)*
            $(($ds, $dty, $ddoc),)*
        }

        $(
            impl From<$dty> for Style {
                fn from(s: $dty) -> Style {
                    Style::$ds(s)
                }
            }
        )*
    }
}

construct_style! {[
    // Box
    (Height, Unit, "Box Height"),
    (Width, Unit, "Box Width"),
    (Padding, Unit, "Box Padding"),
    (Margin, Unit, "Box Margin"),

    // Typo
    (FontWeight, Unit, "Font Weight Style"),
    (FontSize, Unit, "Font Size Style"),
    (FontStretch, Unit, "Font Stretch Style"),
    (LineHeight, Unit, "Line Height Style"),

    // Color
    (Color, Colors, "Custom Color"),
    (BackgroundColor, Colors, "Custom Color"),

    // Flex
    (AlignItems, FlexPosition, "AlignItem Style"),
    (JustifyContent, FlexPosition, "AlignItem Style"),
    (FlexGrow, Unit, "FlexGrow Style"),
    (FlexOrder, Unit, "FlexOrder Style"),
    (Wrap, bool, "Flex Wrap"),

    // Grid
    (GridAutoColumns, GridAuto, "GridAutoColumn Style"),
    (GridAutoRows, GridAuto, "GridAutoColumn Style"),
    (GridAutoFlow, GridFlow, "GridAutoFlow Style"),
    (GridColumnGap, Unit, "GridColumnGap Style"),
    (GridRowGap, Unit, "GridColumnGap Style"),
    (GridTemplateColumns, GridTemplate, "GridTemplateColumns Style"),
    (GridTemplateRows, GridTemplate, "GridTemplateRow Style"),

    // Column
    (ColumnCount, Unit, "ColumnCount Style"),
    (ColumnGap, Unit, "ColumnGap Style"),
    (ColumnRuleColor, Colors, "ColumnRuleColor Style"),
    (ColumnRuleStyle, MultiColumnLineStyle, "ColumnRuleStyle Style"),
], [
    // Flex
    (FlexBasis, FlexBasis, "FlexBasis style"),
    (FlexDirection, FlexDirection, "FlexDirection style"),
    (FlexPosition, FlexPosition, "FlexPosition style"),

    // Grid
    (GridAuto, GridAuto, "Grid Auto Style"),
    (GridFlow, GridFlow, "Grid Flow Style"),
    (GridTemplate, GridTemplate, "Grid Template Style"),
]}
