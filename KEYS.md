## Defaults
As described in the project README, Gfret comes with the following default
keybindings.
| Key | Action |
| --- | --- |
| Ctrl/S | save file |
| Ctrl/Shift/S | save file as |
| Ctrl/E | open with an external program |
| Ctrl/O | load a template from file |
| Ctrl/Shift/P | open the `preferences` dialog |
| Ctrl/A | open the `about` dialog |
| Ctrl/Q | quit the program |

It is possible to configure these keybindings to suit one's own needs by creating
a *keys.toml* file. The location of this file should be in the same directory as
the program's configuration file, usually in `~/.config/gfret`. This file is in
[toml](https://toml.io/en/) format and consists of a simple key/value store.

### Sample *keys.toml* file
```
[keys]
save = "<primary>C"
save_as = "<primary><Shift>C"
```
The first field is the action name, while the value is the key combination
consisting of one or more modifiers and a key value. If the action name is
misspelled or does not refer to a valid action it will be ignored, while if the
keybinding specified does not parse to a valid keybinding the program will fall
back to it's default for that action.
### Actions
| Action name | Description |
| --- | --- |
| open_template | Opens a gfret toml template file |
| save | Save the current file |
| save_as | Save the current document as a new file |
| open_external | Opens the current file in an external program |
| preferences | Open the preferences dialog |
| about | Open the About dialog |
| quit | Close the current window |

### Modifier keys
| Common name | String for *keys.toml* |
| --- | --- |
| Control | &lt;primary&gt; |
| Alt | &lt;Alt&gt; |
| Shift | &lt;Shift&gt; |
| Super (Windows) | &lt;Super&gt; |

### Other key names
All alphabetic keys are simply the key value, in upper or lower case. Number keys
are their numerical value. The arrow keys are "Up", "Down", "Left", and "Right",
which the PgUp and PgDn keys will be "Page_Up" and "Page_Down". The gdk header
file [gdkkeysyms.h](https://gitlab.gnome.org/GNOME/gtk/-/blob/main/gdk/gdkkeysyms.h)
can be consulted for the full list, removing the "GDK_KEY_" portion of the string.
