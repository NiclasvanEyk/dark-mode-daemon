# How It Works

A little peek behind the scenes.
Probably mostly relevant for [Linux](#linux), since there are a thousand different window managers, desktop environments, and the likes.

## MacOS

We integrate directly into the [`DistributedNotificationCenter`](https://developer.apple.com/documentation/foundation/distributednotificationcenter) and listen for the `AppleInterfaceThemeChangedNotification` event.
The current color mode is read from [`UserDefaults`](https://developer.apple.com/documentation/foundation/userdefaults).
For the builtin autostart functionality, we just create a `.plist` file into the `~/Library/LaunchAgents/` directory ([docs](https://support.apple.com/en-gw/guide/terminal/apdc6c1077b-5d5d-4d35-9c19-60f2397b2369/mac)).

## Linux

We use the [`ashpd`](https://docs.rs/ashpd/latest/ashpd) crate, which accesses the XDG portals DBus interfaces.
So if your window manager follows the [XDG Desktop Portal spec](https://flatpak.github.io/xdg-desktop-portal/docs) everything should work fine.
For the builtin autostart functionality, we create a `.desktop` file in the `~/.config/autostart/` directory.
