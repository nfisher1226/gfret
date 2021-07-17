# Gfret
Contents
========
* [Introduction](#introduction)
* [Usage](#usage)
  * [Cli Flags](#cli-flags)
* [Preferences](#preferences)
* [Roadmap](#roadmap)

## Introduction
This is a fretBoard layout tool. Given a set of parameters gfret will
calculate the fret, nut, and bridge locations and output an svg template suitable
for using as a pattern for a fretted stringed instrument. Multiscale designs are
supported. Measurements are currently expected to be in millimeters.
## Building
You will need a Rust toolchain installed, including cargo. Gtk+3x is also
required. To build the program, run ```cargo build --release``` to build a
release binary in target/release.

Alternatively, you can use the included Makefile to build and install the
program, adjusting the installation path with the PREFIX and DESTDIR variables.

## Usage
By default the Gtk based gui will run. If desired, the program can be run in
standalone cli mode by using the ```cli``` subcommand. The first argument to the
cli is the scale length in millimeters. If the -m, or --multiscale argument is
also given, this becomes the bass side scale length.
#### Cli Flags
* -m, --multiscale <scale> - creates a multiscale design where [scale] is
the treble side scale.
* -c, --count <count> - the total number of frets to plot.
* -p, --perpendicular <fret> - which fret is to be perpendicular to the centerline.
* -b, --bridge <spacing> - string spacing at the bridge. This is usually given as
the actual string spacing, while the nut is given as the physical width of the
nut. Therefore, this number is padded by 6mm to give approximately 3mm overhang
of the fretboard in relation to the two outer strings.
* -n, --nut <width> - the width of the nut. On multiscale designs this will be an
approximation, as the nut is slanted.
* -o, --output <file> - the name of the output file.
* -e, --external <program> - open the output file in an external program
## Preferences
Some useful defaults can be set for the rendered image by editing the file
```~/.config/gfret/config.toml```. A sample of this file would look like this:
``` Toml
# An external program which can edit svg files <String>
external_program = "inkscape"
# The border to be placed around the rendered image <f64>
border = 5.0
# The line weight, in mm <f64>
line_weight = 0.5000000000000001
# The color of the fret lines in RGBA format
fretline_color = "rgba(255,255,255,1)"
# The background fill color of the fretboard, in RGBA format
fretboard_color = "rgba(0,0,0,1)"
# Whether or not to draw the dashed centerline <bool>
draw_centerline = true
# The color of the dashed centerline, in RGBA format
centerline_color = "rgba(165,29,45,1)"
# Whether or not to print the specifications used onto the image rendering <bool>
print_specs = true
# The font used to print the specifications
font = "sans 12"
```
If running the gui, all of these settings can be adjusted using the preferences
dialog.
## Roadmap
* For the gui, it would be nice to save state and allow loading specs from and saving
to templates. **partial implementation 4/7/21** | **completed 5/5/21**
* Port to Gtk4
* Support changing from metric to imperial measurements
* Support left handed multiscale fretboards
