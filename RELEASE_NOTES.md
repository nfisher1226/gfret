Contents
========
* [2.0.1](2.0.1-release
* [2.0](2.0-release)

## 2.0.1 release
* Update `fretboard_layout` crate with cleaned up backend code
* Less file IO due to keeping the Config data in a static Mutex
* Where appropriate, the spinbox adjustments now display 2 digits for Metric
  and 3 digits when using Imperial units

## 2.0 release
This is the second major release of *gfret*, bringing a major code cleanup and
reorganization and several new features. The big news is that the interface has
been ported to [Gtk+ version 4](https://drewdevault.com/blog/index.xml). This
necessitated a number of changes to match syntax as well as some rethinking of
the basic assumptions around which the interface had previously been built.

The [fretboard-layout](https://crates.io/crates/fretboard_layout) crate which
serves as the backend to *gfret* has seen significant improvements as well. The
logic which handled anything color related has been moved into a new crate called
[rgba-simple](https://crates.io/crates/rgba_simple). Two new abilities have been
added, the ability to output right or left handed output, and the ability to
output Imperial measurements as well as Metric.
* Port interface to `Gtk4`
  * Port `.glade` files to new `.ui` interface definitions
  * Dialogs moved into a separate module
  * Preferences dialog significantly cleaned up with more consistent padding
    and spacing between elements
  * Dialogs all now use a `headerbar`
  * Main window changed to a `headerbar` and the application menu moved into it
  * Menu ported to new menu definition framework
  * Menu entries and keyboard shortcuts now use the `gtk::SimpleAction` construct
* Enable left handed output
  * Gui changed from a simple `checkbox` enabling the multiscale output to a
    `combobox`.
  * When `Monoscale` is selected, all `Multiscale` related controls are
    completely hidden rather than just greyed out.
  * Internally, the `Specs` struct represents whether the neck is Mono or Multi
    scaled using the `Variant` enum, which has an attached scale length and
    `Handedness` enum when `Multiscale` is selected.
* Enable switching between `Imperial` or `Metric` units
  * The `Config` struct has a new field `units` which is represented by an enum
    `Units`
  * All widget values *and* ranges are adjusted by a factor of 20.4 when
    switching between units
* Revamped `Preferences` dialog
  * Consistent padding and spacing
  * Controls which are no longer valid in the current context are completely
    hidden from the user rather than just being greyed out as before
* New `About` dialog
* New application icon with fretboard and dividers
