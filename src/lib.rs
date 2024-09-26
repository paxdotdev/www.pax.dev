#![allow(unused_imports)]

use pax_kit::*;
use fireworks::*;
use color_picker::*;
use breakout::*;
use space_game::*;

pub mod calculator;
pub use calculator::Calculator;

/* TODO:
[ ] interstitial signup + storage mechanism (db + accounts? + transactional email conf? or airtable / google sheets), 
[ ] designtime API for triggering designer play/pause  
[ ] storyboard / content for GIFs 
[ ] produce GIFs
*/


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


    pub fn show_demo(&mut self, ctx: &NodeContext, _args: Event<Click>) {
        pax_designer::model::perform_action(
            &pax_designer::ProjectMsg(pax_designer::model::ProjectMode::Edit),
            ctx,
        );
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
