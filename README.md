<!--
SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
SPDX-License-Identifier: MIT
-->
<div align="center" markdown="1">

# mars-rover

</div>

`mars-rover` is reimplementation of the venerable and well-known game
[moon-buggy](https://github.com/seehuhn/moon-buggy). The idea of the game is
that you drive a small vehicle over a surface and you have to cope with
obstacles, first there are small ditches you have to jump over (using `Space`),
later there are bigger ones and eventually some aliens appear, which you have
to shoot (using `j`).

<div align="center" markdown="1">

![screenshot](https://raw.githubusercontent.com/b1rger/marsrover/main/data/screenshot1.png)

</div>

## Configuration

The configuration files resides in `$XDG_CONFIG_HOME/marsrover/config.toml`.
You can use it to adapt the colors or create levels.

## Levels

There are a couple of levels predefined in the game, after the last level new
levels are randomly generated.

If you want to define your own levels, you can do that in the configuration
files. Per level there are four settings for probabilities of obstacles
occuring and a `points` setting that defines how many points the user can get
in that level (=the game switches to the next level if the user reaches the
points).

```
[[levels]]
prob_ditch_one = 0.2
prob_ditch_two = 0.0
prob_ditch_three = 0.0
prob_alien = 0.5
points = 100
```