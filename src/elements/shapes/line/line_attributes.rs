use djed::{
    macros::Properties,
    //callback::Callback,
};

#[derive(Clone, PartialEq, Properties)]
pub struct LineProps {
    #[prop_or_default]
    pub x1: String,
    #[prop_or_default]
    pub x2: String,
    #[prop_or_default]
    pub y1: String,
    #[prop_or_default]
    pub y2: String,
    #[prop_or_default]
    pub path_length: String,

    /// Core Attribute
    #[prop_or_default]
    pub id: String,
    /// Core Attribute
    #[prop_or_default]
    pub tabindex: String,

    pub style: String
}