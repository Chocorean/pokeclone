# A Pokemon-inspired first game

[![WASM Build, Test, Deploy](https://github.com/Chocorean/pokeclone/actions/workflows/deploy-wasm.yml/badge.svg)](https://github.com/Chocorean/pokeclone/actions/workflows/deploy-wasm.yml)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-v0.16-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

A pokemon inspired "game" - more like a prototype to be fair. [WASM Demo!](https://chocorean.github.io/pokeclone-demo/)

> [!WARNING]
> This project is my first Bevy public release. It is not meant to be a playable game, but acts more like a demo of what
> is possible to do with Bevy.
>
> I have been working on and off this project for approx. 6 weeks. I know a lot of stuff is super messy, I know it is possible
> to experience weird UI behaviors, the fights suck, something events trigger before the animation stops, but at this point
> I'm tired of this project and would like to move on.
>
> I am very satisfied with the state of the demo, with the gained knowledge and with my work. I am starting to feel burnt out
> and I think it is a good time to start something new.
>
> Moreover, I have ended up writing [my own Bevy plugin](https://github.com/Chocorean/bevy_easy_gif) for the project, which I plan to
> maintain for a while. Even though its use case is super niche, I think it is a nice addition to the ecosystem.

## Dependencies

- Rust tool chain
- `libasound2-dev`
- `libudev-dev`

## Credits

- Heartlessdragoon, for the Pokemon Emerald [exterior tileset](https://www.spriters-resource.com/game_boy_advance/pokemonemerald/asset/61816/)
- Furs The Fox, for the [players sprite sheet](https://www.spriters-resource.com/game_boy_advance/pokemonemerald/asset/8329/)
- SciGho, for the [grass tile sheet](https://ninjikin.itch.io/grass)