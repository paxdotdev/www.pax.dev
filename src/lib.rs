#![allow(unused_imports)]

use pax_kit::*;
use fireworks::*;
use color_picker::*;
use breakout::*;
use space_game::*;

pub mod calculator;
pub use calculator::Calculator;


#[pax]
#[main]
#[file("lib.pax")]
#[has_helpers]
pub struct PaxDotDev {
    pub responsive_stacker_direction: Property<StackerDirection>,
    pub is_mobile: Property<bool>,
}


impl PaxDotDev {

    pub fn handle_mount(&mut self, ctx: &NodeContext) {
        
    }

    pub fn handle_pre_render(&mut self, ctx: &NodeContext) {
        
    }
}

#[helpers]
impl PaxDotDev {
    pub fn get_stacker_direction(is_mobile: bool) -> StackerDirection {
        if is_mobile {
            StackerDirection::Vertical
        } else {
            StackerDirection::Horizontal
        }
    }

    pub fn get_responsive_text_size(is_mobile: bool) -> Size {
        if is_mobile {
            Size::Pixels(16.into())
        } else {
            Size::Pixels(24.into())
        }
    }
}
