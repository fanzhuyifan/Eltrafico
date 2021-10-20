# Eltrafico
NetLimiter-like traffic shaping for Linux

This is a port of https://github.com/cryzed/TrafficToll to rust.

And a gui on top

<img src="./gui.png" width="70%" height="70%">

With network usage:

<img src="./gui_with_network_usage.png" width="70%" height="70%">

# Usage
`eltrafico`

# Howto
Choose the correct interface, and eltrafico will monitor it for active connections

Active program will automatically show up

Choose your limits then confirm with set button

If [bandwhich](https://github.com/imsnif/bandwhich) or [nethogs](https://github.com/raboof/nethogs) is installed on your system, `eltrafico` will use it automatically to show programs live network usage

## Technical details
Eltrafico is split on 2 binaries that communicate through stdin/out:

1- `eltrafico`: create gui and call `bandwhich`/`nethogs` and `eltrafico_tc` as privileged process using pkexec

2- `eltrafico_tc`: traffic shaping, can be controlled via stdin, for the list of commands see https://github.com/sigmaSd/Eltrafico/blob/sudo_isolation/src/eltrafico_tc/main.rs#L252 and https://github.com/sigmaSd/Eltrafico/blob/sudo_isolation/src/eltrafico_tc/main.rs#L79

This allows to run the gui as a normal user, and ask for higher privilege only for `eltrafico_tc` and `bandwhich`/`nethogs` binaries

`eltrafico_tc` needs to be in `$PATH` or you can specify a custom path via `--eltrafico-tc $path_to_binary`

**pkexec usage:**

- pkexec eltrafico_tc
- pkexec bandhwich
- pkexec nethogs
- pkexec pkill nethogs
- pkexec pkill bandwhich

## Current State
Works on my pc (TM)

## Dependencies
 - `iproute2`
 
 **optional:**
 - [nethogs](https://github.com/raboof/nethogs)
 - [bandwhich](https://github.com/imsnif/bandwhich)

## Binary Releases
- Automatic releases by github actions are uploaded here https://github.com/sigmaSd/eltrafico/releases

## Installation
 - needs gtk-dev: https://gtk-rs.org/docs/requirements.html
 - cargo install eltrafico
 
## Building/Dev
- needs gtk-dev: https://gtk-rs.org/docs/requirements.html
- cargo b --bins
- cargo r -- --eltrafico-tc target/debug/eltrafico_tc

## [Changelog](./CHANGELOG.md)
