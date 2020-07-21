use djed::{
    macros::Properties,
    callback::Callback,
};

use web_sys::{MouseEvent};

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
    #[prop_or_default]
    pub clip_path: Option<String>,
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

    // ARIA Attributes
    #[prop_or_default]
    pub aria_activedescendant: Option<String>,
    #[prop_or_default]
    pub aria_atomic: Option<String>,
    #[prop_or_default]
    pub aria_autocomplete: Option<String>,
    #[prop_or_default]
    pub aria_busy: Option<String>,
    #[prop_or_default]
    pub aria_checked: Option<String>,
    #[prop_or_default]
    pub aria_colcount: Option<String>,
    #[prop_or_default]
    pub aria_colindex: Option<String>,
    #[prop_or_default]
    pub aria_colspan: Option<String>,
    #[prop_or_default]
    pub aria_controls: Option<String>,
    #[prop_or_default]
    pub aria_current: Option<String>,
    #[prop_or_default]
    pub aria_describedby: Option<String>,
    #[prop_or_default]
    pub aria_details: Option<String>,
    #[prop_or_default]
    pub aria_disabled: Option<String>,
    #[prop_or_default]
    pub aria_dropeffect: Option<String>,
    #[prop_or_default]
    pub aria_errormessage: Option<String>,
    #[prop_or_default]
    pub aria_expanded: Option<String>,
    #[prop_or_default]
    pub aria_flowto: Option<String>,
    #[prop_or_default]
    pub aria_grabbed: Option<String>,
    #[prop_or_default]
    pub aria_haspopup: Option<String>,
    #[prop_or_default]
    pub aria_hidden: Option<String>,
    #[prop_or_default]
    pub aria_invalid: Option<String>,
    #[prop_or_default]
    pub aria_keyshortcuts: Option<String>,
    #[prop_or_default]
    pub aria_label: Option<String>,
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
    #[prop_or_default]
    pub aria_level: Option<String>,
    #[prop_or_default]
    pub aria_live: Option<String>,
    #[prop_or_default]
    pub aria_modal: Option<String>,
    #[prop_or_default]
    pub aria_multiline: Option<String>,
    #[prop_or_default]
    pub aria_multiselectable: Option<String>,
    #[prop_or_default]
    pub aria_orientation: Option<String>,
    #[prop_or_default]
    pub aria_owns: Option<String>,
    #[prop_or_default]
    pub aria_placeholder: Option<String>,
    #[prop_or_default]
    pub aria_posinset: Option<String>,
    #[prop_or_default]
    pub aria_pressed: Option<String>,
    #[prop_or_default]
    pub aria_readonly: Option<String>,
    #[prop_or_default]
    pub aria_relevant: Option<String>,
    #[prop_or_default]
    pub aria_required: Option<String>,
    #[prop_or_default]
    pub aria_roledescription: Option<String>,
    #[prop_or_default]
    pub aria_rowcount: Option<String>,
    #[prop_or_default]
    pub aria_rowindex: Option<String>,
    #[prop_or_default]
    pub aria_rowspan: Option<String>,
    #[prop_or_default]
    pub aria_selected: Option<String>,
    #[prop_or_default]
    pub aria_setsize: Option<String>,
    #[prop_or_default]
    pub aria_sort: Option<String>,
    #[prop_or_default]
    pub aria_valuemax: Option<String>,
    #[prop_or_default]
    pub aria_valuemin: Option<String>,
    #[prop_or_default]
    pub aria_valuenow: Option<String>,
    #[prop_or_default]
    pub aria_valuetext: Option<String>,
    #[prop_or_default]
    pub role: Option<String>,

/*
    // Animation Event
    pub onbegin: Callback<()>, 
    pub onend: Callback<()>, 
    pub onrepeat: Callback<()>,

    // Document Event
    pub onabort: Callback<()>,
    pub onerror: Callback<()>,
    pub onresize: Callback<()>,
    pub onscroll: Callback<()>,
    pub onunload: Callback<()>,

    // Document Element Event 
    pub oncopy: Callback<()>, 
    pub oncut: Callback<()>, 
    pub onpaste: Callback<()>,

    // Global Event
    pub oncancel: Callback<()>, 
    pub oncanplay: Callback<()>, 
    pub oncanplaythrough: Callback<()>, 
    pub onchange: Callback<()>, */
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>, 
    /*pub onclose: Callback<()>, 
    pub oncuechange: Callback<()>, 
    pub ondblclick: Callback<()>, 
    pub ondrag: Callback<()>, 
    pub ondragend: Callback<()>, 
    pub ondragenter: Callback<()>, 
    pub ondragexit: Callback<()>, 
    pub ondragleave: Callback<()>, 
    pub ondragover: Callback<()>, 
    pub ondragstart: Callback<()>, 
    pub ondrop: Callback<()>, 
    pub ondurationchange: Callback<()>, 
    pub onemptied: Callback<()>, 
    pub onended: Callback<()>, 
    pub onfocus: Callback<()>, 
    pub oninput: Callback<()>, 
    pub oninvalid: Callback<()>, 
    pub onkeydown: Callback<()>, 
    pub onkeypress: Callback<()>, 
    pub onkeyup: Callback<()>, 
    pub onload: Callback<()>, 
    pub onloadeddata: Callback<()>, 
    pub onloadedmetadata: Callback<()>, 
    pub onloadstart: Callback<()>, 
    pub onmousedown: Callback<()>, 
    pub onmouseenter: Callback<()>, 
    pub onmouseleave: Callback<()>, 
    pub onmousemove: Callback<()>, 
    pub onmouseout: Callback<()>, 
    pub onmouseover: Callback<()>, 
    pub onmouseup: Callback<()>, 
    pub onmousewheel: Callback<()>, 
    pub onpause: Callback<()>, 
    pub onplay: Callback<()>, 
    pub onplaying: Callback<()>, 
    pub onprogress: Callback<()>, 
    pub onratechange: Callback<()>, 
    pub onreset: Callback<()>,   
    pub onseeked: Callback<()>, 
    pub onseeking: Callback<()>, 
    pub onselect: Callback<()>, 
    pub onshow: Callback<()>, 
    pub onstalled: Callback<()>, 
    pub onsubmit: Callback<()>, 
    pub onsuspend: Callback<()>, 
    pub ontimeupdate: Callback<()>, 
    pub ontoggle: Callback<()>, 
    pub onvolumechange: Callback<()>, 
    pub onwaiting: Callback<()>, 

    // Graphical Event 
    pub onactivate: Callback<()>, 
    pub onfocusin: Callback<()>, 
    pub onfocusout: Callback<()>, */
}
