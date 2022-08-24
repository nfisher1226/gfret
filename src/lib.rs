#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]

mod actions;
mod application;
/// The cli
pub mod cli;
/// Crate specific errors
mod error;
mod preferences;
mod theme_switcher;
mod window;

pub use {
    actions::Actions, application::Application, error::Error, preferences::PreferencesWindow,
    theme_switcher::ThemeSwitcher, window::Window,
};

/// Switches between imperial and metric units
pub(crate) trait ConvertUnits {
    /// Changes to imperial units
    fn to_imperial(&self);
    /// Changes to metric units
    fn to_metric(&self);
}
