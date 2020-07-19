use djed::{
    macros::Properties,
    //callback::Callback,
};

#[derive(Clone, PartialEq, Properties)]
pub struct LineProps {

    // own Attribute
    /// Defines the x-axis coordinate of the line starting point.
    #[prop_or_default]
    pub x1: Option<String>,
    /// Defines the x-axis coordinate of the line ending point.
    #[prop_or_default]
    pub x2: Option<String>,
    /// Defines the y-axis coordinate of the line starting point.
    #[prop_or_default]
    pub y1: Option<String>,
    /// Defines the y-axis coordinate of the line ending point.
    #[prop_or_default]
    pub y2: Option<String>,
    /// Defines the total path length in user units.
    #[prop_or_default]
    pub path_length: Option<String>,

    // Core
    /// Assigns a unique name to an element.
    #[prop_or_default]
    pub id: Option<String>,
    /// Allows you to control whether an element is focusable and to define 
    /// the relative order of the element for the purposes of sequential focus navigation.
    #[prop_or_default]
    pub tabindex: Option<String>,

    // Styling 
    /// Allows to style an element using CSS declarations.
    #[prop_or_default]
    pub style: Option<String>,
    /// Assigns a class name or set of class names to an element.
    #[prop_or_default]
    pub class: Option<String>,

    // Presentation Attributes
    pub  clip_path: Option<String>,
    #[prop_or_default]
    pub clip_rule: Option<String>,
    #[prop_or_default] 
    pub color: Option<String>,
    #[prop_or_default] 
    pub color_interpolation: Option<String>,
    #[prop_or_default] 
    pub color_rendering: Option<String>,
    #[prop_or_default] 
    pub cursor: Option<String>,
    #[prop_or_default] 
    pub display: Option<String>,
    #[prop_or_default] 
    pub fill: Option<String>,
    #[prop_or_default] 
    pub fill_opacity: Option<String>,
    #[prop_or_default] 
    pub fill_rule: Option<String>,
    #[prop_or_default] 
    pub filter: Option<String>,
    #[prop_or_default] 
    pub mask: Option<String>,
    #[prop_or_default] 
    pub opacity: Option<String>,
    #[prop_or_default] 
    pub pointer_events: Option<String>,
    #[prop_or_default] 
    pub shape_rendering: Option<String>,
    #[prop_or_default] 
    pub stroke: Option<String>,
    #[prop_or_default] 
    pub stroke_dasharray: Option<String>,
    #[prop_or_default] 
    pub stroke_dashoffset: Option<String>,
    #[prop_or_default] 
    pub stroke_linecap: Option<String>,
    #[prop_or_default] 
    pub stroke_linejoin: Option<String>,
    #[prop_or_default] 
    pub stroke_miterlimit: Option<String>,
    #[prop_or_default] 
    pub stroke_opacity: Option<String>,
    #[prop_or_default] 
    pub stroke_width: Option<String>,
    #[prop_or_default] 
    pub transform: Option<String>,
    #[prop_or_default] 
    pub vector_effect: Option<String>,
    #[prop_or_default] 
    pub visibility: Option<String>,

    // Global Event

}
