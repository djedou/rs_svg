use djed::{
    macros::Properties,
    //callback::Callback,
};
use serde::Serialize;

#[derive(Clone, PartialEq, Properties, Serialize)]
pub struct LineProps {
    pub x1: String,
    pub x2: String,
    pub y1: String,
    pub y2: String,
    pub path_length: String,

    /// Core Attribute
    pub id: String,
    /// Core Attribute
    pub tabindex: String,

    pub style: String,

    pub stroke: String,
    pub stroke_width: String,
    pub fill: String
}