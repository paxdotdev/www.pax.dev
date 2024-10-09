#![allow(unused_imports)]

use pax_kit::*;

#[pax]
#[main]
#[file("lib.pax")]
#[has_helpers]
pub struct PaxDotDev {
    pub responsive_stacker_direction: Property<StackerDirection>,
    pub is_mobile: Property<bool>,
    pub show_desktop_warning: Property<bool>,
}


impl PaxDotDev {

    pub fn handle_mount(&mut self, ctx: &NodeContext) {
        
    }

    pub fn hide_desktop_warning_click(&mut self, ctx: &NodeContext, _args: Event<Click>) {
        self.show_desktop_warning.set(false);
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
