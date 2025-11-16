# Dark Mode Daemon

Run scripts when the system color scheme changes between light and dark.

## Getting Started

The easiest way is to download and run the official installation script

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/niclasvaneyk/dark-mode-daemon/releases/latest/download/dark-mode-daemon-installer.sh | sh
```

Then you can run the watch process by running

```
dark-mode-daemon
```

This won't do much, since you probably haven't set up any scripts yet.
Dark Mode Daemon runs every executable file in  `~/.config/dark-mode-daemon/scripts/` and sets the `DMD_COLOR_MODE` environment variable to either `light` or `dark`.
This lets you adjust configuration files, other environment variables, or whatever else you can come up with.
Head over to the [list of recipes](./docs/recipes.md) for inspiration.
Examples include adding automatic color adjustments for [Alacritty](./docs/recipes.md#alacritty), [Helix](./docs/recipes.md#helix), [Fish](./docs/recipes.md#fish), and more.

You likely want to have Dark Mode Daemon launch in the background when you log into your user.
There are many solutions to this, but we've prepared a builtin solution that should work for most platforms

```
dark-mode-daemon autostart setup
```

If you are interested in how this or the color mode detection works, have a look at our [behind the scenes documentation](./docs/how-it-works.md).

