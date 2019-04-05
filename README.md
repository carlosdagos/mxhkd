# mxhkd (alpha)

## Description

_mxhkd_ is an X daemon that reacts to input events by executing commands.

It's like [_sxhkd_](https://github.com/baskerville/sxhkd) but it's modal
rather than using chords.

## Modal key daemon

Initially the idea was to write a full compositor that would be modal.
Kind of like `i3` but modal. That proved to be too hard and it was
obviously going to take months. Instead this was cranked out in a few
days.

### Why?

#### Help with RSI

Key chords like those used in _sxhkd_ and many other programs can
contribute to RSI. Using modes is a great way to reuse the keys
closest to the natural positioning of your hands.

[Read up on the _emacs pinky_ this doesn't make any sense to
you.](https://en.wikipedia.org/wiki/Emacs#Emacs_pinky)

#### Also

The initial intention was to do modal control of the windowing.
That turned into running arbitrary commands, and use any window
manager you want. `bspwm` might be a natural fit to use with _mxhkd_,
but also something like `herbstluftwm`. Up to you. Testing was mostly
done with `i3` and that worked fine as well.

## Documentation

Docs can be found [here](https://carlosdagos.github.io/mxhkd).

### Sample configuration

See the [`examples` folder](./examples).

## Runnning

If it's installed:

```bash
mxhkd --config /path/to/config.toml
```

### Runtime dependencies

There's a runtime dependency in `xmodmap`. This will be removed
in the near future. It's used at startup to determine the key code
to character layout.

### Running in dev mode with Nix

Clone this repo, go to the root folder, and then run

``` nix
nix-shell --command "cargo run -- --config examples/mxhkd_config.toml"
```

Have a look at the `shell.nix` file to see which system
dependencies are needed.

## Compatibility

_mxhkd_ takes over your main X window, so it might not be compatible with
any other system that intends to do the same. In normal `Window` mode, it
simply listens on the whichever _mode toggle key_ has been configured, and
passes through the rest of the keystrokes to whichever other program is
listening to commands. So technically it shouldn't interfere with many
things. In `Normal` mode it listens to your entire keyboard, as you
would expect.

### Keyboard layouts

The `xmodmap` dependency might not work as expected depending on the keyboard.
Please open an issue if something completely wrong happens.

## Super important

This thing is **totally hacked**.

**Use at your own risk!**

Happy to receive PRs.

## Authors

* Carlos D'Agostino ([@carlosdagos](https://github.com/carlosdagos))
* Robert McMichael ([@robbiemcmichael](https://github.com/robbiemcmichael))
