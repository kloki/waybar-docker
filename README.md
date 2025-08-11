# waybar-docker

A docker module for [waybar](https://github.com/Alexays/Waybar) that works for me.

Install

```bash
cargo install waybar-docker
```

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
