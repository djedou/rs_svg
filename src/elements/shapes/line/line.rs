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
    stroke: Option<String>,
    stroke_width: Option<String>,
    fill: Option<String>
}

pub enum State {
    Clicked,
}


impl Component for Line {
    type State = State;
    type Props = LineProps;

    fn create(props: Self::Props, _link: ComponentLink<Self>) -> Self {
        Line {
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
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            fill: props.fill
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
        self.stroke = props.stroke;
        self.stroke_width = props.stroke_width;
        self.fill = props.fill;


        true
    }
/*
    fn rendered(&mut self, _first_render: bool) {
        //let line = self.node_ref.cast::<SvgLineElement>().unwrap();
        self.line = Some(line);
    }
*/

    fn view(&self) -> Html {
        /*let p = LineBasic { 
            x1: self.x1.clone().into(),
            x2: self.x2.clone().into(),
            y1: self.y1.clone().into(),
            y2: self.y2.clone().into(),
        };*/
     
        html! {
            LineBasic {
                x1: self.x1.clone(),
                x2: self.x2.clone(),
                /*y1: self.y1.clone(),
                y2: self.y2.clone(),
                path_length: self.path_length.clone(),
                id: self.id.clone(),
                tabindex: self.tabindex.clone(),
                style: self.style.clone(),
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width.clone(),
                fill: self.fill.clone(),*/
            }
        }  
    }
}

/*
 /*<line 
    x1 = self.x1
    x2 = self.x2
    y1 = self.y1
    y2 = self.y2
    pathLength = self.path_length
    id = self.id
    tabindex = self.tabindex
    style = self.style
    stroke = self.stroke
    stroke_width = self.stroke_width
    fill = self.fill
                
            />*/ 
*/
impl Line {
    fn set_attributes(&mut self, props: LineProps) {


        /*let line = self.line.clone().unwrap();
        //line
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
        }*/

        
    }
}


markup::define! {
    LineBasic(
        x1: Option<String>,
        x2: Option<String>,
        /*y1: Option<String>,
        y2: Option<String>,
        path_length: Option<String>,
        id: Option<String>,
        tabindex: Option<String>,
        style: Option<String>,
        stroke: Option<String>,
        stroke_width: Option<String>,
        fill: Option<String>,*/
    )
    {
        line [
                x1 = x1,
                x2 = x2,
                /*y1 = y1,
                y2 = y2,
                pathLength = path_length,
                id = id,
                tabindex = tabindex,
                style = style,
                stroke = stroke,
                stroke_width = stroke_width,
                fill = fill*/
            ] {}
    }
}

/*x1 = props.x1,
            x2 = props.x2,
            y1 = props.y1,
            y2 = props.y2,
            pathLength = props.path_length,
            id = props.id,
            tabindex = props.tabindex,
            style = props.style,
            stroke = props.stroke,
            stroke_width = props.stroke_width,
            fill = props.fill*/