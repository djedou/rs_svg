use djed::{
    html, 
    djed::{Component, ComponentLink, Html, ShouldRender},
    callback::Callback,
    djed::listener::{*},
    callback::Callback,
};

use std::rc::Rc;
use super::line_attributes::LineProps;
use crate::utils::{set_attribute, set_event};
use web_sys::{MouseEvent};

/// Core Attribute
/// id, tabindex, style
pub struct SvgLine {
    //link: ComponentLink<Self>,
    x1: Option<String>,
    x2: Option<String>,
    y1: Option<String>,
    y2: Option<String>,
    path_length: Option<String>,

    id: Option<String>,
    tabindex: Option<String>,

    style: Option<String>,
    class: Option<String>,

    clip_path: Option<String>,
    clip_rule: Option<String>, 
    color: Option<String>, 
    color_interpolation: Option<String>, 
    color_rendering: Option<String>, 
    cursor: Option<String>, 
    display: Option<String>, 
    fill: Option<String>, 
    fill_opacity: Option<String>, 
    fill_rule: Option<String>, 
    filter: Option<String>, 
    mask: Option<String>, 
    opacity: Option<String>, 
    pointer_events: Option<String>, 
    shape_rendering: Option<String>, 
    stroke: Option<String>, 
    stroke_dasharray: Option<String>, 
    stroke_dashoffset: Option<String>, 
    stroke_linecap: Option<String>, 
    stroke_linejoin: Option<String>, 
    stroke_miterlimit: Option<String>, 
    stroke_opacity: Option<String>, 
    stroke_width: Option<String>, 
    transform: Option<String>, 
    vector_effect: Option<String>, 
    visibility: Option<String>,

    aria_activedescendant: Option<String>,
    aria_atomic: Option<String>,
    aria_autocomplete: Option<String>,
    aria_busy: Option<String>,
    aria_checked: Option<String>,
    aria_colcount: Option<String>,
    aria_colindex: Option<String>,
    aria_colspan: Option<String>,
    aria_controls: Option<String>,
    aria_current: Option<String>,
    aria_describedby: Option<String>,
    aria_details: Option<String>,
    aria_disabled: Option<String>,
    aria_dropeffect: Option<String>,
    aria_errormessage: Option<String>,
    aria_expanded: Option<String>,
    aria_flowto: Option<String>,
    aria_grabbed: Option<String>,
    aria_haspopup: Option<String>,
    aria_hidden: Option<String>,
    aria_invalid: Option<String>,
    aria_keyshortcuts: Option<String>,
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
    aria_level: Option<String>,
    aria_live: Option<String>,
    aria_modal: Option<String>,
    aria_multiline: Option<String>,
    aria_multiselectable: Option<String>,
    aria_orientation: Option<String>,
    aria_owns: Option<String>,
    aria_placeholder: Option<String>,
    aria_posinset: Option<String>,
    aria_pressed: Option<String>,
    aria_readonly: Option<String>,
    aria_relevant: Option<String>,
    aria_required: Option<String>,
    aria_roledescription: Option<String>,
    aria_rowcount: Option<String>,
    aria_rowindex: Option<String>,
    aria_rowspan: Option<String>,
    aria_selected: Option<String>,
    aria_setsize: Option<String>,
    aria_sort: Option<String>,
    aria_valuemax: Option<String>,
    aria_valuemin: Option<String>,
    aria_valuenow: Option<String>,
    aria_valuetext: Option<String>,
    role: Option<String>,

    // Animation Event
    onbegin: Option<Callback<MouseEvent>>,
    onend: Option<Callback<MouseEvent>>, 
    onrepeat: Option<Callback<MouseEvent>>,

    // Document Event
    onabort: Option<Callback<MouseEvent>>,
    onerror: Option<Callback<MouseEvent>>,
    onresize: Option<Callback<MouseEvent>>,
    onscroll: Option<Callback<MouseEvent>>,
    onunload: Option<Callback<MouseEvent>>,

    // Document Element Event 
    oncopy: Option<Callback<MouseEvent>>, 
    oncut: Option<Callback<MouseEvent>>, 
    onpaste: Option<Callback<MouseEvent>>,

    // Global Event
    oncancel: Option<Callback<MouseEvent>>, 
    oncanplay: Option<Callback<MouseEvent>>, 
    oncanplaythrough: Option<Callback<MouseEvent>>, 
    onchange: Option<Callback<MouseEvent>>,
    onclick: Option<Callback<MouseEvent>>, 
    onclose: Option<Callback<MouseEvent>>,
    oncuechange: Option<Callback<MouseEvent>>,
    ondblclick: Option<Callback<MouseEvent>>,
    ondrag: Option<Callback<MouseEvent>>,
    ondragend: Option<Callback<MouseEvent>>,
    ondragenter: Option<Callback<MouseEvent>>,
    ondragexit: Option<Callback<MouseEvent>>,
    ondragleave: Option<Callback<MouseEvent>>,
    ondragover: Option<Callback<MouseEvent>>,
    ondragstart: Option<Callback<MouseEvent>>,
    ondrop: Option<Callback<MouseEvent>>,
    ondurationchange: Option<Callback<MouseEvent>>,
    onemptied: Option<Callback<MouseEvent>>,
    onended: Option<Callback<MouseEvent>>,
    onfocus: Option<Callback<MouseEvent>>,
    oninput: Option<Callback<MouseEvent>>,
    oninvalid: Option<Callback<MouseEvent>>,
    onkeydown: Option<Callback<MouseEvent>>,
    onkeypress: Option<Callback<MouseEvent>>,
    onkeyup: Option<Callback<MouseEvent>>,
    onload: Option<Callback<MouseEvent>>,
    onloadeddata: Option<Callback<MouseEvent>>,
    onloadedmetadata: Option<Callback<MouseEvent>>,
    onloadstart: Option<Callback<MouseEvent>>,
    onmousedown: Option<Callback<MouseEvent>>,
    onmouseenter: Option<Callback<MouseEvent>>,
    onmouseleave: Option<Callback<MouseEvent>>,
    onmousemove: Option<Callback<MouseEvent>>,
    onmouseout: Option<Callback<MouseEvent>>,
    onmouseover: Option<Callback<MouseEvent>>,
    onmouseup: Option<Callback<MouseEvent>>,
    onmousewheel: Option<Callback<MouseEvent>>,
    onpause: Option<Callback<MouseEvent>>,
    onplay: Option<Callback<MouseEvent>>,
    onplaying: Option<Callback<MouseEvent>>,
    onprogress: Option<Callback<MouseEvent>>,
    onratechange: Option<Callback<MouseEvent>>,
    onreset: Option<Callback<MouseEvent>>,
    onseeked: Option<Callback<MouseEvent>>,
    onseeking: Option<Callback<MouseEvent>>,
    onselect: Option<Callback<MouseEvent>>,
    onshow: Option<Callback<MouseEvent>>,
    onstalled: Option<Callback<MouseEvent>>,
    onsubmit: Option<Callback<MouseEvent>>,
    onsuspend: Option<Callback<MouseEvent>>,
    ontimeupdate: Option<Callback<MouseEvent>>,
    ontoggle: Option<Callback<MouseEvent>>,
    onvolumechange: Option<Callback<MouseEvent>>,
    onwaiting: Option<Callback<MouseEvent>>, 

    // Graphical Event 
    onactivate: Option<Callback<MouseEvent>>,
    onfocusin: Option<Callback<MouseEvent>>,
    onfocusout: Option<Callback<MouseEvent>>,

}

pub enum State {

}


impl Component for SvgLine {
    type State = State;
    type Props = LineProps;

    fn create(props: Self::Props, _link: ComponentLink<Self>) -> Self {
        SvgLine {
            //link,
            x1: props.x1,
            x2: props.x2,
            y1: props.y1,
            y2: props.y2,
            path_length: props.path_length, 

            id: props.id,
            tabindex: props.tabindex,

            style: props.style,
            class: props.class,

            clip_path: props.clip_path,
            clip_rule: props.clip_rule, 
            color: props.color, 
            color_interpolation: props.color_interpolation,
            color_rendering: props.color_rendering, 
            cursor: props.cursor, 
            display: props.display, 
            fill: props.fill, 
            fill_opacity: props.fill_opacity, 
            fill_rule: props.fill_rule, 
            filter: props.filter, 
            mask: props.mask, 
            opacity: props.opacity, 
            pointer_events: props.pointer_events, 
            shape_rendering: props.shape_rendering, 
            stroke: props.stroke, 
            stroke_dasharray: props.stroke_dasharray, 
            stroke_dashoffset: props.stroke_dashoffset, 
            stroke_linecap: props.stroke_linecap, 
            stroke_linejoin: props.stroke_linejoin, 
            stroke_miterlimit: props.stroke_miterlimit, 
            stroke_opacity: props.stroke_opacity, 
            stroke_width: props.stroke_width, 
            transform: props.transform, 
            vector_effect: props.vector_effect, 
            visibility: props.visibility,

            aria_activedescendant: props.aria_activedescendant,
            aria_atomic: props.aria_atomic,
            aria_autocomplete: props.aria_autocomplete,
            aria_busy: props.aria_busy,
            aria_checked: props.aria_checked,
            aria_colcount: props.aria_colcount,
            aria_colindex: props.aria_colindex,
            aria_colspan: props.aria_colspan,
            aria_controls: props.aria_controls,
            aria_current: props.aria_current,
            aria_describedby: props.aria_describedby,
            aria_details: props.aria_details,
            aria_disabled: props.aria_disabled,
            aria_dropeffect: props.aria_dropeffect,
            aria_errormessage: props.aria_errormessage,
            aria_expanded: props.aria_expanded,
            aria_flowto: props.aria_flowto,
            aria_grabbed: props.aria_grabbed,
            aria_haspopup: props.aria_haspopup,
            aria_hidden: props.aria_hidden,
            aria_invalid: props.aria_invalid,
            aria_keyshortcuts: props.aria_keyshortcuts,
            aria_label: props.aria_label,
            aria_labelledby: props.aria_labelledby,
            aria_level: props.aria_level,
            aria_live: props.aria_live,
            aria_modal: props.aria_modal,
            aria_multiline: props.aria_multiline,
            aria_multiselectable: props.aria_multiselectable,
            aria_orientation: props.aria_orientation,
            aria_owns: props.aria_owns,
            aria_placeholder: props.aria_placeholder,
            aria_posinset: props.aria_posinset,
            aria_pressed: props.aria_pressed,
            aria_readonly: props.aria_readonly,
            aria_relevant: props.aria_relevant,
            aria_required: props.aria_required,
            aria_roledescription: props.aria_roledescription,
            aria_rowcount: props.aria_rowcount,
            aria_rowindex: props.aria_rowindex,
            aria_rowspan: props.aria_rowspan,
            aria_selected: props.aria_selected,
            aria_setsize: props.aria_setsize,
            aria_sort: props.aria_sort,
            aria_valuemax: props.aria_valuemax,
            aria_valuemin: props.aria_valuemin,
            aria_valuenow: props.aria_valuenow,
            aria_valuetext: props.aria_valuetext,
            role: props.role,


            onclick: props.onclick, 
            
        }
    }

    fn update(&mut self, _state: Self::State) -> ShouldRender {
        
        false
    }

    fn change(&mut self, props: Self::Props) -> ShouldRender {
        self.x1 = props.x1;
        self.x2 = props.x2;
        self.y1 = props.y1;
        self.y2 = props.y2;
        self.path_length = props.path_length; 

        self.id = props.id;
        self.tabindex = props.tabindex;

        self.style = props.style;
        self.class = props.class;

        self.clip_path = props.clip_path;
        self.clip_rule = props.clip_rule; 

        self.color = props.color; 
        self.color_interpolation = props.color_interpolation;
        self.color_rendering = props.color_rendering;

        self.cursor = props.cursor; 
        self.display = props.display; 

        self.fill = props.fill; 
        self.fill_opacity = props.fill_opacity; 
        self.fill_rule = props.fill_rule; 

        self.filter = props.filter; 
        self.mask = props.mask; 
        self.opacity = props.opacity; 
        self.pointer_events = props.pointer_events; 
        self.shape_rendering = props.shape_rendering; 

        self.stroke = props.stroke; 
        self.stroke_dasharray = props.stroke_dasharray; 
        self.stroke_dashoffset = props.stroke_dashoffset; 
        self.stroke_linecap = props.stroke_linecap; 
        self.stroke_linejoin = props.stroke_linejoin; 
        self.stroke_miterlimit = props.stroke_miterlimit; 
        self.stroke_opacity = props.stroke_opacity; 
        self.stroke_width = props.stroke_width; 
        
        self.transform = props.transform; 
        self.vector_effect = props.vector_effect; 
        self.visibility = props.visibility;

        self.aria_activedescendant = props.aria_activedescendant;
        self.aria_atomic = props.aria_atomic;
        self.aria_autocomplete = props.aria_autocomplete;
        self.aria_busy = props.aria_busy;
        self.aria_checked = props.aria_checked;
        self.aria_colcount = props.aria_colcount;
        self.aria_colindex = props.aria_colindex;
        self.aria_colspan = props.aria_colspan;
        self.aria_controls = props.aria_controls;
        self.aria_current = props.aria_current;
        self.aria_describedby = props.aria_describedby;
        self.aria_details = props.aria_details;
        self.aria_disabled = props.aria_disabled;
        self.aria_dropeffect = props.aria_dropeffect;
        self.aria_errormessage = props.aria_errormessage;
        self.aria_expanded = props.aria_expanded;
        self.aria_flowto = props.aria_flowto;
        self.aria_grabbed = props.aria_grabbed;
        self.aria_haspopup = props.aria_haspopup;
        self.aria_hidden = props.aria_hidden;
        self.aria_invalid = props.aria_invalid;
        self.aria_keyshortcuts = props.aria_keyshortcuts;
        self.aria_label = props.aria_label;
        self.aria_labelledby = props.aria_labelledby;
        self.aria_level = props.aria_level;
        self.aria_live = props.aria_live;
        self.aria_modal = props.aria_modal;
        self.aria_multiline = props.aria_multiline;
        self.aria_multiselectable = props.aria_multiselectable;
        self.aria_orientation = props.aria_orientation;
        self.aria_owns = props.aria_owns;
        self.aria_placeholder = props.aria_placeholder;
        self.aria_posinset = props.aria_posinset;
        self.aria_pressed = props.aria_pressed;
        self.aria_readonly = props.aria_readonly;
        self.aria_relevant = props.aria_relevant;
        self.aria_required = props.aria_required;
        self.aria_roledescription = props.aria_roledescription;
        self.aria_rowcount = props.aria_rowcount;
        self.aria_rowindex = props.aria_rowindex;
        self.aria_rowspan = props.aria_rowspan;
        self.aria_selected = props.aria_selected;
        self.aria_setsize = props.aria_setsize;
        self.aria_sort = props.aria_sort;
        self.aria_valuemax = props.aria_valuemax;
        self.aria_valuemin = props.aria_valuemin;
        self.aria_valuenow = props.aria_valuenow;
        self.aria_valuetext = props.aria_valuetext;
        self.role = props.role;

        // Animation Event
        /*self.onbegin = props.onbegin; 
        self.onend = props.onend; 
        self.onrepeat = props.onrepeat;

        // Document Event
        self.onabort = props.onabort;
        self.onerror = props.onerror;
        self.onresize = props.onresize;
        self.onscroll = props.onscroll;
        self.onunload = props.onunload;

        // Document Element Event 
        self.oncopy = props.oncopy; 
        self.oncut = props.oncut; 
        self.onpaste = props.onpaste;

        // Global Event
        self.oncancel = props.oncancel; 
        self.oncanplay = props.oncanplay; 
        self.oncanplaythrough = props.oncanplaythrough; 
        self.onchange = props.onchange; */
        self.onclick = props.onclick; 
        /*self.onclose = props.onclose; 
        self.oncuechange = props.oncuechange; 
        self.ondblclick = props.ondblclick; 
        self.ondrag = props.ondrag; 
        self.ondragend = props.ondragend; 
        self.ondragenter = props.ondragenter; 
        self.ondragexit = props.ondragexit; 
        self.ondragleave = props.ondragleave; 
        self.ondragover = props.ondragover; 
        self.ondragstart = props.ondragstart; 
        self.ondrop = props.ondrop; 
        self.ondurationchange = props.ondurationchange; 
        self.onemptied = props.onemptied; 
        self.onended = props.onended; 
        self.onfocus = props.onfocus; 
        self.oninput = props.oninput; 
        self.oninvalid = props.oninvalid; 
        self.onkeydown = props.onkeydown; 
        self.onkeypress = props.onkeypress; 
        self.onkeyup = props.onkeyup; 
        self.onload = props.onload; 
        self.onloadeddata = props.onloadeddata; 
        self.onloadedmetadata = props.onloadedmetadata; 
        self.onloadstart = props.onloadstart; 
        self.onmousedown = props.onmousedown; 
        self.onmouseenter = props.onmouseenter; 
        self.onmouseleave = props.onmouseleave; 
        self.onmousemove = props.onmousemove; 
        self.onmouseout = props.onmouseout; 
        self.onmouseover = props.onmouseover; 
        self.onmouseup = props.onmouseup; 
        self.onmousewheel = props.onmousewheel; 
        self.onpause = props.onpause; 
        self.onplay = props.onplay; 
        self.onplaying = props.onplaying; 
        self.onprogress = props.onprogress; 
        self.onratechange = props.onratechange; 
        self.onreset = props.onreset;   
        self.onseeked = props.onseeked; 
        self.onseeking = props.onseeking; 
        self.onselect = props.onselect; 
        self.onshow = props.onshow; 
        self.onstalled = props.onstalled; 
        self.onsubmit = props.onsubmit; 
        self.onsuspend = props.onsuspend; 
        self.ontimeupdate = props.ontimeupdate; 
        self.ontoggle = props.ontoggle; 
        self.onvolumechange = props.onvolumechange; 
        self.onwaiting = props.onwaiting; 

        // Graphical Event 
        self.onactivate = props.onactivate; 
        self.onfocusin = props.onfocusin; 
        self.onfocusout = props.onfocusout; */


        true
    }

    fn view(&self) -> Html {

        let mut line_tag = html! {<line />};

        set_attribute(self.x1.as_ref(), &mut line_tag, "x1");
        set_attribute(self.x2.as_ref(), &mut line_tag, "x2");
        set_attribute(self.y1.as_ref(), &mut line_tag, "y1");
        set_attribute(self.y2.as_ref(), &mut line_tag, "y2");
        set_attribute(self.path_length.as_ref(), &mut line_tag, "pathLength");

        set_attribute(self.id.as_ref(), &mut line_tag, "id");
        set_attribute(self.tabindex.as_ref(), &mut line_tag, "tabindex");

        set_attribute(self.style.as_ref(), &mut line_tag, "style");
        set_attribute(self.class.as_ref(), &mut line_tag, "class");

        set_attribute(self.clip_path.as_ref(), &mut line_tag, "clip-path");
        set_attribute(self.clip_rule.as_ref(), &mut line_tag, "clip-rule");

        set_attribute(self.color.as_ref(), &mut line_tag, "color");
        set_attribute(self.color_interpolation.as_ref(), &mut line_tag, "color-interpolation");
        set_attribute(self.color_rendering.as_ref(), &mut line_tag, "color-rendering");

        set_attribute(self.cursor.as_ref(), &mut line_tag, "cursor");
        set_attribute(self.display.as_ref(), &mut line_tag, "display");

        set_attribute(self.fill.as_ref(), &mut line_tag, "fill");
        set_attribute(self.fill_opacity.as_ref(), &mut line_tag, "fill-opacity");
        set_attribute(self.fill_rule.as_ref(), &mut line_tag, "fill-rule");

        set_attribute(self.filter.as_ref(), &mut line_tag, "filter");
        set_attribute(self.mask.as_ref(), &mut line_tag, "mask");
        set_attribute(self.opacity.as_ref(), &mut line_tag, "opacity");
        set_attribute(self.pointer_events.as_ref(), &mut line_tag, "pointer-events");
        set_attribute(self.shape_rendering.as_ref(), &mut line_tag, "shape-rendering");
        
        set_attribute(self.stroke.as_ref(), &mut line_tag, "stroke");
        set_attribute(self.stroke_dasharray.as_ref(), &mut line_tag, "stroke-dasharray");
        set_attribute(self.stroke_dashoffset.as_ref(), &mut line_tag, "stroke-dashoffset");
        set_attribute(self.stroke_linecap.as_ref(), &mut line_tag, "stroke-linecap");
        set_attribute(self.stroke_linejoin.as_ref(), &mut line_tag, "stroke-linejoin");
        set_attribute(self.stroke_miterlimit.as_ref(), &mut line_tag, "stroke-miterlimit");
        set_attribute(self.stroke_opacity.as_ref(), &mut line_tag, "stroke-opacity");
        set_attribute(self.stroke_width.as_ref(), &mut line_tag, "stroke-width");

        set_attribute(self.transform.as_ref(), &mut line_tag, "transform");
        set_attribute(self.vector_effect.as_ref(), &mut line_tag, "vector-effect");
        set_attribute(self.visibility.as_ref(), &mut line_tag, "visibility");

        set_attribute(self.aria_activedescendant.as_ref(), &mut line_tag, "aria-activedescendant");
        set_attribute(self.aria_atomic.as_ref(), &mut line_tag, "aria-atomic");
        set_attribute(self.aria_autocomplete.as_ref(), &mut line_tag, "aria-autocomplete");
        set_attribute(self.aria_busy.as_ref(), &mut line_tag,"aria-busy");
        set_attribute(self.aria_checked .as_ref(), &mut line_tag, "aria-checked");
        set_attribute(self.aria_colcount.as_ref(), &mut line_tag, "aria-colcount");
        set_attribute(self.aria_colindex.as_ref(), &mut line_tag, "aria-colindex");
        set_attribute(self.aria_colspan .as_ref(), &mut line_tag, "aria-colspan");
        set_attribute(self.aria_controls .as_ref(), &mut line_tag, "aria-controls");
        set_attribute(self.aria_current .as_ref(), &mut line_tag, "aria-current");
        set_attribute(self.aria_describedby.as_ref(), &mut line_tag, "aria_-escribedby");
        set_attribute(self.aria_details.as_ref(), &mut line_tag, "aria-details");
        set_attribute(self.aria_disabled.as_ref(), &mut line_tag, "aria-disabled");
        set_attribute(self.aria_dropeffect.as_ref(), &mut line_tag, "aria-dropeffect");
        set_attribute(self.aria_errormessage.as_ref(), &mut line_tag, "aria-errormessage");
        set_attribute(self.aria_expanded.as_ref(), &mut line_tag, "aria-expanded");
        set_attribute(self.aria_flowto.as_ref(), &mut line_tag, "aria-flowto");
        set_attribute(self.aria_grabbed.as_ref(), &mut line_tag, "aria-grabbed");
        set_attribute(self.aria_haspopup.as_ref(), &mut line_tag, "aria-haspopup");
        set_attribute(self.aria_hidden.as_ref(), &mut line_tag, "aria-hidden");
        set_attribute(self.aria_invalid.as_ref(), &mut line_tag, "aria-invalid");
        set_attribute(self.aria_keyshortcuts.as_ref(), &mut line_tag, "aria-keyshortcuts");
        set_attribute(self.aria_label.as_ref(), &mut line_tag, "aria-label");
        set_attribute(self.aria_labelledby.as_ref(), &mut line_tag, "aria-labelledby");
        set_attribute(self.aria_level.as_ref(), &mut line_tag, "aria-level");
        set_attribute(self.aria_live.as_ref(), &mut line_tag, "aria-live");
        set_attribute(self.aria_modal.as_ref(), &mut line_tag, "aria-modal");
        set_attribute(self.aria_multiline.as_ref(), &mut line_tag, "aria-multiline");
        set_attribute(self.aria_multiselectable.as_ref(), &mut line_tag, "aria-multiselectable");
        set_attribute(self.aria_orientation.as_ref(), &mut line_tag, "aria-orientation");
        set_attribute(self.aria_owns.as_ref(), &mut line_tag,"aria-owns");
        set_attribute(self.aria_placeholder.as_ref(), &mut line_tag, "aria-placeholder");
        set_attribute(self.aria_posinset.as_ref(), &mut line_tag, "aria-posinset");
        set_attribute(self.aria_pressed.as_ref(), &mut line_tag, "aria-pressed");
        set_attribute(self.aria_readonly.as_ref(), &mut line_tag, "aria-readonly");
        set_attribute(self.aria_relevant.as_ref(), &mut line_tag, "aria-relevant");
        set_attribute(self.aria_required.as_ref(), &mut line_tag, "aria-required");
        set_attribute(self.aria_roledescription.as_ref(), &mut line_tag, "aria-roledescription");
        set_attribute(self.aria_rowcount.as_ref(), &mut line_tag, "aria-rowcount");
        set_attribute(self.aria_rowindex.as_ref(), &mut line_tag, "aria-rowindex");
        set_attribute(self.aria_rowspan.as_ref(), &mut line_tag, "aria-rowspan");
        set_attribute(self.aria_selected.as_ref(), &mut line_tag, "aria-selected");
        set_attribute(self.aria_setsize.as_ref(), &mut line_tag, "aria-setsize");
        set_attribute(self.aria_sort.as_ref(), &mut line_tag, "aria-sort");
        set_attribute(self.aria_valuemax.as_ref(), &mut line_tag, "aria-valuemax");
        set_attribute(self.aria_valuemin.as_ref(), &mut line_tag, "aria-valuemin");
        set_attribute(self.aria_valuenow.as_ref(), &mut line_tag, "aria-valuenow");
        set_attribute(self.aria_valuetext.as_ref(), &mut line_tag, "aria-valuetext");
        set_attribute(self.role.as_ref(), &mut line_tag, "role");

        

        if let Some(event) = self.onclick.as_ref() {
            let onclick_listener = onclick::Wrapper::new(event.clone());
            set_event(Some(event),&mut line_tag, Rc::new(onclick_listener));
        }
        /*<button onclick=self.link.callback(|_| Msg::Clicked)>
                { &self.title }
            </button>*/

        line_tag

    }
}
