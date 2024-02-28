# Dark Mode Daemon

Run scripts when the system color scheme changes between light and dark.

## Installation

For now, simply use [Homebrew](https://brew.sh) is the easiest option:

```shell
# 1. Install the binary
brew install NiclasvanEyk/dark-mode-daemon/dark-mode-daemon

# 2. Launch the daemon when logging in
brew services start dark-mode-daemon
```

## Usage

Place executable scripts in one of the following locations:

  - `$XDG_CONFIG_HOME/dark-mode-daemon/scripts`
  - `$HOME/.config/dark-mode-daemon/scripts`

You can use the following as an example

```bash
#!/usr/bin/env bash

say "Changed to $DMD_COLOR_MODE mode"
```

Dont forget to make it executable (`chmod +x myscript.sh`)!
Then, if not automatically run at startup through Homebrew, run

```shell
dark-mode-daemon daemon
```

to begin listening for color mode changes.

If you used the example, you should hear the phrase "Changed to dark mode".

