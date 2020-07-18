use djed::{
    macros::Properties,
    //callback::Callback,
};


#[derive(Clone, PartialEq, Properties)]
pub struct CoreAttrProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub tabindex: String,
}