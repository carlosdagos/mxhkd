# mxhkd -- Modal X Key Daemon

## SYNOPSIS

`mxhkd` --config `<config>`

`mxhkd` --version

`mxhkd` --help

## DESCRIPTION

`mxhkd` is an X program that reacts to input events by executing commands.
It listens for a `mode_switch` specification, which can be either a single
key or a key plus a modifier. The configuration specifies bindings which
when detected will run commands specified by the user.

There are two modes, and one pseudo-mode:

- `Window`: In this mode, `mxhkd` only listens on the `mode_switch`
  specification, and allows all other keypress events to pass through.

- `Normal`: This mode is triggered by `mxhkd` when it detects that
  `mode_switch` was pressed by the user. In this mode `mxhkd` listens to
  all key presses waiting for a `binding` to be attempted. It exits back
  into `Window` mode regardless of successfully running a binding or not.

- `Normal + Sticky`: This is a pseudo mode which is optional. In the default
  `Normal` mode, `mxhkd` will exit back into `Window` mode after a successful
  or unsuccessful command. However in `Sticky` it will stay in `Normal` mode
  waiting for bindings. This is a useful command e.g. if the user is
  repeating the same command over and over, such as resizing windows or
  the changing the volume.

The intention of `mxhkd` is that it is `modal`, in order to discourage the
use of key chords. In some circumstances, simple key chords are acceptable.
For example, a `mode_switch` could be specified as

```
mode_switch = { key = "space", modifier = "Alt" }
```

This will only trigger `mxhkd` to go into `Normal` mode when `Alt+Space`
is presssed by the user. In general, the user should choose an unintrusive
key to switch modes.

Another example can be for the bindings:

```
h  = "i3 focus left"
j  = "i3 focus down"
k  = "i3 focus up"
l  = "i3 focus right"
f  = "i3 fullscreen toggle"

H  = "i3 resize grow   width  5 px or 5 ppt"
J  = "i3 resize shrink height 5 px or 5 ppt"
K  = "i3 resize grow   height 5 px or 5 ppt"
L  = "i3 resize shrink width  5 px or 5 ppt"
F  = "i3 floating toggle"
```

The top commands refer to the letters `h`, `j`, `k`, `l`, `f` pressed
normally (that is, without simultaneously pressing any other keys).
The uppercase notation refers to the same letters pressed while
pressing `Shift`. `mxhkd` can differenciate between these two in order
to overload the keystrokes while still focusing on comfort.

## CONFIGURATION

The configuration file for `mxhkd` is in `toml`. There are two top-level
keys to configure: `settings` and `bindings`, which are detailed below.

`settings`: Controls the overall behavior of `mxhkd`.

  * `shell` (**required**): The shell in which all `mxhkd` commands
    are executed. `bash` and `fish` have been tested. In principle, any
    shell that can accept a `-c` command will work.

  * `mode_switch` (**required**): The key specification used to switch
    modes. This will work in both `Normal` and `Window` mode. Examples can be

    ```
    mode_switch = { key = "space", modifier = "Alt" }
    ```

    This will trigger `mode_switch` when `Alt+Space` is pressed by the user.

    Another example is

    ```
    mode_switch = { key = "Caps_Lock" }
    ```

    This will trigger `mode_switch` when `Caps_Lock` is pressed. This would
    work nicely if for example the key has been reconfigured to disable
    setting all caps.

  * `mode_change_cmd` (**optional**): Command that `mxhkd` will run every time
    the `mode` changes. This setting will replace the string `%{mode}%` with
    the corresponding mode.

  * `not_found_cmd` (**optional**): Command that `mxhkd` will run every time
    a binding is attempted but not found in the configuration. Recommended.
    `mxhkd` will replace the string `%{binding}%` with the failed binding
    issued by the user.

  * `sticky_mode` (**optional**): Will enable the `Normal + Sticky` pseudo
    mode. When pressed. This key will only be triggered once in `Normal`
    mode. In `Window` mode it has no effect.

`bindings`: Defines the custom bindings attached to commands. For example:

```
    a  = "rofi -show window"
    o  = "rofi -show run"
    t  = "termite"

    sx = "spotify-controls play-pause"
    sn = "spotify-controls next"
    sp = "spotify-controls prev"

    sl = "i3lock -i ~/Downloads/mountains_bg.png"
```

For this sample configuration, a user who wants to lock the screen using
`i3lock` would first press `s` followed by `l`; `mxhkd` will trigger
the command immediately.

**NOTE**: Overlapping bindings are not supported. No binding should be
the prefix of another binding.

## OPTIONS

`--config`: Specifies the location of the configuration file and starts
`mxhkd` with it.


`--help`: Shows the help menu.


`--version`: Outputs the program version.

## SYNTAX

`mxhkd` uses a configuration file in the `toml` format. Refer
to the example in the source code.

## BUGS

`mxhkd` has a runtime dependency in `xmodmap`. This will go away in
the future. It's used at startup to determine the key code to
character layout.

Please file any bugs encountered using the following
[link](https://github.com/carlosdagos/mxhkd/issues)

## HISTORY

* April 2019: v0.1

## AUTHOR

Carlos D'Agostino <m@cdagostino.io>

Robbie McMichael <none@none.com>

## COPYRIGHT

See LICENSE file in the source code.

## SEE ALSO

[mxhkd](https://github.com/carlosdagos/mxhkd)

[sxhkd](https://github.com/baskerville/sxhkd)

[toml](https://github.com/toml-lang/toml)
