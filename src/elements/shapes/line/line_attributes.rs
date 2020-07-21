use djed::{
    macros::Properties,
    callback::Callback,
    djed::listener::{ChangeData, InputData},
};

use web_sys::{MouseEvent, AnimationEvent, Event, DragEvent,
    FocusEvent, KeyboardEvent, ProgressEvent,
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


    // Animation Event
    #[prop_or_default]
    pub onbegin: Option<Callback<AnimationEvent>>,
    #[prop_or_default]
    pub onend: Option<Callback<AnimationEvent>>, 
    #[prop_or_default]
    pub onrepeat: Option<Callback<AnimationEvent>>,

    // Document Event
    #[prop_or_default]
    pub onabort: Option<Callback<Event>>,
    #[prop_or_default]
    pub onerror: Option<Callback<Event>>,
    #[prop_or_default]
    pub onresize: Option<Callback<Event>>,
    #[prop_or_default]
    pub onscroll: Option<Callback<Event>>,
    #[prop_or_default]
    pub onunload: Option<Callback<Event>>,

    // Document Element Event 
    #[prop_or_default]
    pub oncopy: Option<Callback<Event>>,
    #[prop_or_default] 
    pub oncut: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onpaste: Option<Callback<Event>>,

    // Global Event
    #[prop_or_default]
    pub oncancel: Option<Callback<Event>>,
    #[prop_or_default] 
    pub oncanplay: Option<Callback<Event>>,
    #[prop_or_default] 
    pub oncanplaythrough: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onchange: Option<Callback<ChangeData>>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onclose: Option<Callback<Event>>,
    #[prop_or_default] 
    pub oncuechange: Option<Callback<Event>>,
    #[prop_or_default] 
    pub ondblclick: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub ondrag: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragend: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragenter: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragexit: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragleave: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragover: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragstart: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondrop: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondurationchange: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onemptied: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onended: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onfocus: Option<Callback<FocusEvent>>,
    #[prop_or_default] 
    pub oninput: Option<Callback<InputData>>,
    #[prop_or_default] 
    pub oninvalid: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onkeydown: Option<Callback<KeyboardEvent>>,
    #[prop_or_default] 
    pub onkeypress: Option<Callback<KeyboardEvent>>,
    #[prop_or_default] 
    pub onkeyup: Option<Callback<KeyboardEvent>>,
    #[prop_or_default] 
    pub onload: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onloadeddata: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onloadedmetadata: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onloadstart: Option<Callback<ProgressEvent>>,
    #[prop_or_default] 
    pub onmousedown: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseenter: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseleave: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmousemove: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseout: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseover: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseup: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmousewheel: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onpause: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onplay: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onplaying: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onprogress: Option<Callback<ProgressEvent>>,
    #[prop_or_default] 
    pub onratechange: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onreset: Option<Callback<Event>>,
    #[prop_or_default]   
    pub onseeked: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onseeking: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onselect: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onshow: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onstalled: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onsubmit: Option<Callback<FocusEvent>>,
    #[prop_or_default] 
    pub onsuspend: Option<Callback<Event>>,
    #[prop_or_default] 
    pub ontimeupdate: Option<Callback<Event>>,
    #[prop_or_default] 
    pub ontoggle: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onvolumechange: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onwaiting: Option<Callback<Event>>, 

    // Graphical Event 
    #[prop_or_default]
    pub onactivate: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onfocusin: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onfocusout: Option<Callback<Event>>, 
}
