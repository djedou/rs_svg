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
