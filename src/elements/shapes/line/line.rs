use djed::{
    html, 
    djed::{Component, ComponentLink, Html, ShouldRender},
    //callback::Callback,
    //macros::Properties
};
use super::line_attributes::LineProps;

pub struct Line {
    //link: ComponentLink<Self>,
    //title: String,
    //onsignal: Callback<()>,
    x1: String,
    x2: String,
    y1: String,
    y2: String,
    path_length: String,
    id: String,
    tabindex: String,
}

pub enum State {
    Clicked,
}


impl Component for Line {
    type State = State;
    type Props = LineProps;

    fn create(props: Self::Props, _link: ComponentLink<Self>) -> Self {
        Line {
            //link,
            //title: props.title,
            //onsignal: props.onsignal,
            x1: props.x1,
            x2: props.x2,
            y1: props.y1,
            y2: props.y2,
            path_length: props.path_length,
            id: props.id,
            tabindex: props.tabindex,
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
        //self.title = props.title;
        //self.onsignal = props.onsignal;
        self.x1 = props.x1;
        self.x2 = props.x2;
        self.y1 = props.y1;
        self.y2 = props.y2;
        self.path_length = props.path_length;
        self.id = props.id;
        self.tabindex = props.tabindex;


        true
    }

    fn view(&self) -> Html {
        html! {
            <line
                x1 = self.x1
                x2 = self.x2
                y1 = self.y1
                y2 = self.y2
                pathLength = self.path_length
                id = self.id
                tabindex = self.tabindex
            />
        }
    }
}