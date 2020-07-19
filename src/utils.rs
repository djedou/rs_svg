
use djed::{
    djed_dom::VNode,
};

use std::fmt::Display;

pub fn set_attribute<'a>(attr: Option<&String>, tagget: &mut VNode, attr_name: &'a str) {

    if let Some(data) = attr {
        if let VNode::VTag(tag) = tagget {
            tag.add_attribute(attr_name, data);
        }
    }

}
