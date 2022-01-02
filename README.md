# quad

Made with [Bevy](https://github.com/bevyengine/bevy), a Rust game engine!

I could've used [Macroquad](https://macroquad.rs/) (and what a naming intrigue that would be!) but for my laziness, hence the fork*

Simply meant to demonstrate features I like

Feel free to mention preferences in issues or use this code in your own software (so long as you mind the license)

\*originally forked from [8bit-pudding/bevy-tetris](https://github.com/8bit-pudding/bevy-tetris)
___

TODO:
* Implement a proper rotation system (no matter what I implement, at the moment I don't see a way to avoid copious amounts of hardcoding, which I'm loath to, so this will probably take a while)
* Add original features

MAYBE-TODO:
* Website

NOT-TODO:
* Cross-platform installation: out of scope for this project
___

Licensed under the [MIT License](https://opensource.org/licenses/MIT)

Disclaimer (inspired by that of Quinn): This software is in no way affiliated with or sponsored by The Tetris Company, nor is it part of their Tetris line of products
___

CAVEAT:
* .cargo/config: Custom linker and unstable share-generics feature, both of which improve performance, are disabled by default
* Remember to use 'cargo run --release'/'cargo build --release' if you're a user
