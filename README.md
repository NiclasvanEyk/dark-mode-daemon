# Dark Mode Daemon

Run scripts when the system color scheme changes between light and dark.

## Installation

For now, simply use [Homebrew](https://brew.sh) is the easiest option:

```shell
# 1. Install the binary
brew install NiclasvanEyk/dark-mode-daemon/dark-mode-daemon

# 2. (optional) Launch the daemon on system start
brew services start dark-mode-daemon
```

Note that if you don't run the second step, you can still manually watch for changes using `dark-mode-daemon daemon`.

## Usage

Create a new directory for scripts that should be run when changing color modes:

```shell
mkdir $HOME/.config/dark-mode-daemon/scripts
```

> Alternatively use `$XDG_CONFIG_HOME` instead of `$HOME/.config` if you configured it

Then create as many scripts there as you like, but don't forget to make them executable.
They will be automatically be run when changing between dark and light mode.
The `DMD_COLOR_MODE` environment variable will be either set to `light` or `dark`, depending on the new mode.

### Example

The original motivation for creating this program was to sync theming environment variables, such as [difftastics `DFT_BACKGROUND`](https://github.com/Wilfred/difftastic) or [bats `BAT_THEME`](https://github.com/sharkdp/bat) with the current operating system color scheme.
But lets use a more impractical example.

MacOS includes `say`, a text-to-speech program available on the command line.
We can use this to loudly announce our dark mode changes.

First we create the script 

```shell
touch $HOME/.config/dark-mode-daemon/scripts/announce.sh
```

Then fill it with the following content

```shell
#!/usr/bin/env bash

say "Changed to $DMD_COLOR_MODE mode"
```

finally make it executable

```shell
chmod +x $HOME/.config/dark-mode-daemon/scripts/announce.sh
```

You can verify that it will be run using

```shell
dark-mode-daemon list
```

this should print something like the following:

```
📂 Using scripts in /Users/youruser/.config/dark-mode-daemon/scripts...

/Users/youruser/.config/dark-mode-daemon/scripts/announce.sh
```

Now turn your volume up, toggle dark mode, and be amazed at the result.
Or create more useful scripts that adjust your terminal emulator, vim/emacs/editor theme, or something totally different.
