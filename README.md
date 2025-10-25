# waybar-docker

A docker module for [waybar](https://github.com/Alexays/Waybar) that works for me.

## Installation

Check [Releases](https://github.com/kloki/waybar-docker/releases) for binaries and installers

Add this to you `config.jsonc`

```json
{
  "custom/docker": {
    "exec": "~/.cargo/bin/waybar-docker",
    "return-type": "json",
    "interval": 5
  }
}
```
