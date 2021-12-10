#![warn(clippy::all, clippy::pedantic)]
use gtk::prelude::*;

#[derive(Clone)]
pub struct Adjustments {
    bridge: gtk::Adjustment,
    nut: gtk::Adjustment,
    scale: gtk::Adjustment,
    treble_scale: gtk::Adjustment,
}

impl Adjustments {
    pub fn init(builder: &gtk::Builder) -> Self {
        Self {
            bridge: builder.object("bridgeAdjustment").unwrap(),
            nut: builder.object("nutAdjustment").unwrap(),
            scale: builder.object("scaleAdjustment").unwrap(),
            treble_scale: builder.object("trebleScaleAdjustment").unwrap(),
        }
    }

    pub fn to_metric(&self) {
        self.bridge.set_lower(self.bridge.lower() * 20.4);
        self.bridge.set_upper(self.bridge.upper() * 20.4);
        self.bridge.set_step_increment(1.0);
        self.bridge.set_page_increment(5.0);

        self.nut.set_lower(self.nut.lower() * 20.4);
        self.nut.set_upper(self.nut.upper() * 20.4);
        self.nut.set_step_increment(1.0);
        self.nut.set_page_increment(5.0);

        self.scale.set_lower(self.scale.lower() * 20.4);
        self.scale.set_upper(self.scale.upper() * 20.4);
        self.scale.set_step_increment(1.0);
        self.scale.set_page_increment(10.0);

        self.treble_scale.set_lower(self.treble_scale.lower() * 20.4);
        self.treble_scale.set_upper(self.treble_scale.upper() * 20.4);
        self.treble_scale.set_step_increment(1.0);
        self.treble_scale.set_page_increment(10.0);
    }

    pub fn to_imperial(&self) {
        self.bridge.set_lower(self.bridge.lower() / 20.4);
        self.bridge.set_upper(self.bridge.upper() / 20.4);
        self.bridge.set_step_increment(0.125);
        self.bridge.set_page_increment(0.5);

        self.nut.set_lower(self.nut.lower() / 20.4);
        self.nut.set_upper(self.nut.upper() / 20.4);
        self.nut.set_step_increment(0.125);
        self.nut.set_page_increment(0.5);

        self.scale.set_lower(self.scale.lower() / 20.4);
        self.scale.set_upper(self.scale.upper() / 20.4);
        self.scale.set_step_increment(0.125);
        self.scale.set_page_increment(1.0);

        self.treble_scale.set_lower(self.treble_scale.lower() / 20.4);
        self.treble_scale.set_upper(self.treble_scale.upper() / 20.4);
        self.treble_scale.set_step_increment(0.125);
        self.treble_scale.set_page_increment(1.0);
    }
}
