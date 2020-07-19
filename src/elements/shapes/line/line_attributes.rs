use djed::{
    macros::Properties,
    //callback::Callback,
};
use serde::Serialize;

#[derive(Clone, PartialEq, Properties, Serialize)]
pub struct LineProps {
    #[prop_or("".into())]
    pub x1: String,
    #[prop_or("".into())]
    pub x2: String,
    #[prop_or("".into())]
    pub y1: String,
    #[prop_or("".into())]
    pub y2: String,
    #[prop_or("".into())]
    pub path_length: String,

    /// Core Attribute
    #[prop_or("".into())]
    pub id: String,
    /// Core Attribute
    #[prop_or("".into())]
    pub tabindex: String,
    #[prop_or("".into())]
    pub style: String,
    #[prop_or("".into())]
    pub stroke: String,
    #[prop_or("".into())]
    pub stroke_width: String,
    #[prop_or("".into())]
    pub fill: String
}