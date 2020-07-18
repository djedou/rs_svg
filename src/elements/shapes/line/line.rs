use djed::{
    html, 
    djed::{Component, ComponentLink, Html,NodeRef, ShouldRender},
    //callback::Callback,
    //macros::Properties
};
use super::line_attributes::LineProps;
use crate::utils::get_field_by_name;


use web_sys::{SvgElement, SvgTextElement, SvggElement, SvgLineElement};

/// Line component can have the following Attributes:: 
/// x1, x2, y1, y2, path_length,

/// Core Attribute
/// id, tabindex, style
pub struct Line {
    //link: ComponentLink<Self>,
    line: Option<SvgLineElement>,
    node_ref: NodeRef,
}

pub enum State {
    Clicked,
}


impl Component for Line {
    type State = State;
    type Props = LineProps;

    fn create(_props: Self::Props, _link: ComponentLink<Self>) -> Self {
        Line {
            line: None,
            node_ref: NodeRef::default(),
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
        self.set_attributes(props);
        true
    }

    fn rendered(&mut self, _first_render: bool) {
        let line = self.node_ref.cast::<SvgLineElement>().unwrap();
        self.line = Some(line);
    }


    fn view(&self) -> Html {
        html! {
            <line />
            
        }
    }
}

impl Line {
    fn set_attributes(&mut self, props: LineProps) {
        let line = self.line.clone().unwrap();
        // set x1
        let x1_value: Option<String> = get_field_by_name(props.clone(), "x1");
        match x1_value {
            Some(x1) => {
                let _ = line.set_attribute("x1", x1.as_ref());
            },
            None => {}
        }

        let x2_value: Option<String> = get_field_by_name(props.clone(), "x2");
        match x2_value {
            Some(x2) => {
                let _ = line.set_attribute("x2", x2.as_ref());
            },
            None => {}
        }

        let y1_value: Option<String> = get_field_by_name(props.clone(), "y1");
        match y1_value {
            Some(y1) => {
                let _ = line.set_attribute("y1", y1.as_ref());
            },
            None => {}
        }


        let y2_value: Option<String> = get_field_by_name(props.clone(), "y2");
        match y2_value {
            Some(y2) => {
                let _ = line.set_attribute("y2", y2.as_ref());
            },
            None => {}
        }

        let stroke_value: Option<String> = get_field_by_name(props.clone(), "stroke");
        match stroke_value {
            Some(stroke) => {
                let _ = line.set_attribute("stroke", stroke.as_ref());
            },
            None => {}
        }

        let stroke_width_value: Option<String> = get_field_by_name(props.clone(), "stroke_width");
        match stroke_width_value {
            Some(stroke_width) => {
                let _ = line.set_attribute("stroke-width", stroke_width.as_ref());
            },
            None => {}
        }

        let fill_value: Option<String> = get_field_by_name(props.clone(), "fill");
        match fill_value {
            Some(fill) => {
                let _ = line.set_attribute("fill", fill.as_ref());
            },
            None => {}
        }
    }
}
