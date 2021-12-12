# Gfret
<!-- cargo-sync-readme start -->

Contents
========
* [Introduction](#introduction)
* [Usage](#usage)
  * [Cli](#running-the-command-line-interface)
  * [Gui](#running-the-gui)
    * [Keybindings](#keybindings)
  * [Templates](#templates)
* [Configuration](#configuration)
* [Building](#building)
* [Roadmap](#roadmap)

## Introduction
Gfret renders an svg image template of a fretboard for a stringed instrument.
It has a Gtk interface as well as a command line interface and can produce
templates for instruments ranging from a piccolo mandolin to an upright bass.
Multiscale designs are also supported. Currently, all measurements are
expressed in metric units only.
## Running the gui
Calling the program by invoking ```gfret``` without any arguments will run
the Gtk interface. Additionally, a .desktop file and icon are included and
will be installed if the program is installed using the included
```Makefile```, and can be used for launching the program from desktop menus
or creating shortcuts.
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
## Keybindings
| Key | Action |
| --- | --- |
| Ctrl/S | save file |
| Ctrl/Shift/S | save file as |
| Ctrl/E | open with an external program |
| Ctrl/O | load a template from file |
| Ctrl/Shift/P | open the preferences dialog |
| Ctrl/Q | quit the program |
## Templates
Along with the svg output, Gfret will save the specifications used to
generate the rendering in a Toml file with it's name corresponding to the
name of the svg file. These templates can be loaded later, either as an
argument when invoking the program, in which case the output will be
immediately generated, or else loaded from the Gui interface for further
editing. This is useful for sharing a common scale among multiple designs to
use as a starting point.
## Configuration
On Unix systems the default configuration directory is ```~/.config/gfret```.
Gfret will maintain a configuration file here in [Toml](https://github.com/toml-lang/toml)
format, with the following fields:
```Toml
## must match either "Metric" or "Imperial"
units = "Metric"
## the command line to run
external_program = "inkscape"
## the size of the border around the image
border = 10.0
## how thick the lines are
line_weight = 1.0

[fretline_color]
## Colors must be "Hex", "Reduced" or "RGBA"
## "Reduced" will take a whole number between 0 and 255 for each channel,
## while "RGBA" takes a decimal between 0.0 and 1.0.

## "Hex" will have a <color> field and an <alpha> field
## ColorType = "Hex"
## color = "#00ff00"
## alpha = 1.0
ColorType = "Reduced"
red = <u8>
green = <u8>
blue = <u8>
alpha = <u8>

[fretboard_color]
ColorType = "Reduced"
red = 36
green = 31
blue = 49
alpha = 255

[centerline_color]
ColorType = "Reduced"
red = 0
green = 0
blue = 255
alpha = 255

## Fonts take a family and weight (style), but size is ignored
[font]
family = "Sans"
weight = "Normal"
```
> Note: The graphical interface has a preferences dialog and will take care
> of maintaining the preferences file for you. There will be no need to edit
> this file by hand in normal use.
## Building
You will need a Rust toolchain installed, including cargo. Gtk+3x is also
required. To build the program, run ```cargo build --release``` to build a
release binary in target/release.

Alternatively, you can use the included Makefile to build and install the
program, adjusting the installation path with the PREFIX and DESTDIR variables.

## Roadmap
* For the gui, it would be nice to save state and allow loading specs from and saving
to templates. **partial implementation 4/7/21** | **completed 5/5/21**
* Port to Gtk4 **completed 12/21**
* Support changing from metric to imperial measurements **completed 12/21**
* Support left handed multiscale fretboards **completed 12/21**
<!-- cargo-sync-readme end -->
