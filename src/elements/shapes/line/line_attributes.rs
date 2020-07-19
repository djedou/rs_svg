use djed::{
    macros::Properties,
    //callback::Callback,
};
use serde::Serialize;

#[derive(Clone, PartialEq, Properties, Serialize)]
pub struct LineProps {

    // own Attribute
    /// Defines the x-axis coordinate of the line starting point.
    #[prop_or_default]
    pub x1: String,
    /// Defines the x-axis coordinate of the line ending point.
    #[prop_or_default]
    pub x2: String,
    /// Defines the y-axis coordinate of the line starting point.
    #[prop_or_default]
    pub y1: String,
    /// Defines the y-axis coordinate of the line ending point.
    #[prop_or_default]
    pub y2: String,
    /// Defines the total path length in user units.
    #[prop_or_default]
    pub path_length: String,

    // Core
    /// assigns a unique name to an element.
    #[prop_or_default]
    pub id: String,
    /// allows you to control whether an element is focusable and to define 
    /// the relative order of the element for the purposes of sequential focus navigation.
    #[prop_or_default]
    pub tabindex: String,

    // Styling 
    /// allows to style an element using CSS declarations.
    #[prop_or_default]
    pub style: String,
    /// Assigns a class name or set of class names to an element.
    #[prop_or_default]
    pub class: String,


    #[prop_or_default]
    pub stroke: String,
    #[prop_or_default]
    pub stroke_width: String,
    #[prop_or_default]
    pub fill: String
}