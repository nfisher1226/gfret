# Gfret
Contents
========
* [Introduction](#introduction)
* [Getting Gfret](#getting-gfret)
* [Usage](#usage)
  * [Cli](#running-the-command-line-interface)
  * [Gui](#running-the-gui)
    * [Keybindings](#keybindings)
  * [Templates](#templates)
* [Configuration](#configuration)
* [Building](#building)
* [Roadmap](#roadmap)

## Introduction
*Gfret* renders an svg image template of a fretboard for a stringed instrument.
It has a Gtk interface as well as a command line interface and can produce
templates for instruments ranging from a piccolo mandolin to an upright bass.
Multiscale designs are also supported. Currently, all measurements are
expressed in metric units only.
## Getting Gfret
*Gfret* is distributed primarily in source form. The main repo is at
[codeberg.org](https://codeberg.org/jeang3nie/gfret) with mirrors at
[gitlab](https://gitlab.com/jeang3nie/gfret) and
[github.com](https://github.com/nfisher1226/gfret). Releases are recommended.
## Usage
*Gfret* can be used from the command line or using the Gtk+ interface.
## Running the command line interface
```Bash
gfret-cli

Output an svg without running the interface

USAGE:
   gfret cli [OPTIONS] [OUTPUT]

ARGS:
   <OUTPUT>    Name of the output file [default: output.svg]

OPTIONS:
   -b, --bridge <BRIDGE>
           Bridge spacing [default: 56]

   -c, --count <COUNT>
           Total fret count [default: 24]

   -e, --external <EXTERN>
           Open output file in external program [default: inkscape]

   -h, --help
           Print help information

   -l, --left
           Multiscale fretboard reversed (left handed)

   -m, --multi <MULTI>
           Creates a multiscale fretboard with <MULTI> as the treble scale. [default: 610]

   -n, --nut <NUT>
           Nut width [default: 43]

   -o, --output <OUTPUT>
           Name of the output file [default: output.svg]

   -p, --perpendicular <PERPENDICULAR>
           Set which fret is perpendicular to the centerline [default: 8]
   -s, --scale <SCALE>
           Scale length in mm [default: 648]
```
## Running the gui
Calling the program by invoking ```gfret``` without any arguments will run
the Gtk interface. Additionally, a .desktop file and icon are included and
will be installed if the program is installed using the included
```Makefile```, and can be used for launching the program from desktop menus
or creating shortcuts.
## Keybindings
> Note: See [KEYS.md](KEYS.md) for information on changing keybindings

| Key | Action |
| --- | --- |
| Ctrl/S | save file |
| Ctrl/Shift/S | save file as |
| Ctrl/E | open with an external program |
| Ctrl/O | load a template from file |
| Ctrl/Shift/P | open the `preferences` dialog |
| Ctrl/A | open the `about` dialog |
| Ctrl/Q | quit the program |
## Building
You will need a Rust toolchain installed, including cargo. Gtk+4x is also
required.
```sh
cargo build --release
```
If desired, a release distribution can then be generated which will include the
binary, svg and png icons, Unix man pages, XDG .desktop file, gschema.xml and
shell completions.
```sh
cargo xtask dist
```
To install, copy the contents of `target/dist` to the appropriate prefix and
compile the gschemas.
```Sh
cp -Rv target/dist/* /usr/local
glib-compile-schemas /usr/local/share/glib-2.0/gschemas/
```
## Roadmap
- [x] For the gui, it would be nice to save state and allow loading specs
  from and saving to templates. **partial implementation 4/7/21** |
  **completed 5/5/21**
- [x] Port to Gtk4 **completed 12/21**
- [x] Support changing from metric to imperial measurements **completed 12/21**
- [x] Support left handed multiscale fretboards **completed 12/21**
- [x] Orient left handed output with bass strings on top **completed 1/22**
### 3.0
- [x] Subclass application window from AdwApplicationWindow
- [x] Subclass GfretApplication from AdwApplication
- [x] Create a gio::Settings object and store application state in it
- [x] Move Config into GfretApplication
- [x] Provide theme switcher
- [x] Subclass preferences window from AdwPreferencesWindow
- [x] Make a property action and menu entry for Units
- [x] Add AdwToastOverlay and set toast when a file is saved
- [x] Replace gfret::File type for glib::File
- [x] Make a `changed` boolean property of GfretWindow to track saved state
- [x] Create all dialogs as needed instead of persisting for the program lifespan
- [x] Replace templates with the ability to re-open svg files created by this
      application
- [ ] Prepare for FlatPak packaging and distribution
