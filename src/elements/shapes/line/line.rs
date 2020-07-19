use djed::{
    html, 
    djed::{Component, ComponentLink, Html, ShouldRender},
    djed_dom::{VNode},
};
use super::line_attributes::LineProps;
use crate::utils::set_attribute;


/// Core Attribute
/// id, tabindex, style
pub struct SvgLine {
    //link: ComponentLink<Self>,
    //line: Option<SvgLineElement>,
    //node_ref: NodeRef,
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

}

pub enum State {
    Clicked,
}


impl Component for SvgLine {
    type State = State;
    type Props = LineProps;

    fn create(props: Self::Props, _link: ComponentLink<Self>) -> Self {
        SvgLine {
            //line: None,
            //node_ref: NodeRef::default(),
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
        }
    }

    fn update(&mut self, msg: Self::State) -> ShouldRender {
        match msg {
            State::Clicked => {
                //self.onsignal.emit(());
            }
        }
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


        true
    }

    fn view(&self) -> Html {

        let mut line_tag = html! {<line />};

        /*if let Some(x1_data) = self.x1.as_ref() {
            if let VNode::VTag(tag) = &mut line_tag {
                tag.add_attribute("x1", x1_data);
            }
        }*/
        // pub fn set_attribute<'a, T: Display>(attr: &Option<T>, tagget: &mut VNode, attr_name: &'a str) {
  
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
        set_attribute(self.fill_rule.as_ref(), &mut line_tag, "class");
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

        /*if let Some(x2_data) = self.x2.as_ref() {
            if let VNode::VTag(tag) = &mut line_tag {
                tag.add_attribute("x2", x2_data);
            }
        }

        if let Some(y1_data) = self.y1.as_ref() {
            if let VNode::VTag(tag) = &mut line_tag {
                tag.add_attribute("y1", y1_data);
            }
        }

        if let Some(y2_data) = self.y2.as_ref() {
            if let VNode::VTag(tag) = &mut line_tag {
                tag.add_attribute("y2", y2_data);
            }
        }

        if let Some(path_length_data) = self.path_length.as_ref() {
            if let VNode::VTag(tag) = &mut line_tag {
                tag.add_attribute("pathLength", path_length_data);
            }
        }

        if let Some(id_data) = self.id.as_ref() {
            if let VNode::VTag(tag) = &mut line_tag {
                tag.add_attribute("id", id_data);
            }
        }

        if let Some(tabindex_data) = self.tabindex.as_ref() {
            if let VNode::VTag(tag) = &mut line_tag {
                tag.add_attribute("tabindex", tabindex_data);
            }
        }

        if let Some(style_data) = self.style.as_ref() {
            if let VNode::VTag(tag) = &mut line_tag {
                tag.add_attribute("style", style_data);
            }
        }

        if let Some(class_data) = self.class.as_ref() {
            if let VNode::VTag(tag) = &mut line_tag {
                tag.add_attribute("class", class_data);
            }
        }*/


        line_tag

    }
}
