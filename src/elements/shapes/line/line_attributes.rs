use djed::{
    macros::Properties,
    //callback::Callback,
};
use serde::Serialize;

#[derive(Clone, PartialEq, Properties, Serialize)]
pub struct LineProps {

    // own Attribute
    /// Defines the x-axis coordinate of the line starting point.
    #[prop_or(None)]
    pub x1: Option<String>,
    /// Defines the x-axis coordinate of the line ending point.
    #[prop_or(None)]
    pub x2: Option<String>,
    /// Defines the y-axis coordinate of the line starting point.
    #[prop_or(None)]
    pub y1: Option<String>,
    /// Defines the y-axis coordinate of the line ending point.
    #[prop_or(None)]
    pub y2: Option<String>,
    /// Defines the total path length in user units.
    #[prop_or(None)]
    pub path_length: Option<String>,

    // Core
    /// assigns a unique name to an element.
    #[prop_or(None)]
    pub id: Option<String>,
    /// allows you to control whether an element is focusable and to define 
    /// the relative order of the element for the purposes of sequential focus navigation.
    #[prop_or(None)]
    pub tabindex: Option<String>,

    // Styling 
    /// allows to style an element using CSS declarations.
    #[prop_or(None)]
    pub style: Option<String>,
    /// Assigns a class name or set of class names to an element.
    #[prop_or(None)]
    pub class: Option<String>,


    #[prop_or(None)]
    pub stroke: Option<String>,
    #[prop_or(None)]
    pub stroke_width: Option<String>,
    #[prop_or(None)]
    pub fill: Option<String>

    // Global Event

}