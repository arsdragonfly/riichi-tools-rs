# riichi-tools-rs
v0.0.47

[![Build Status](https://travis-ci.org/harphield/riichi-tools-rs.png?branch=master)](https://travis-ci.org/harphield/riichi-tools-rs)

A tool library for riichi mahjong written in Rust, made mostly to be used as a WASM component.

Lots of inspiration from other mahjong tools, like https://euophrys.itch.io/mahjong-efficiency-trainer, http://tenhou.net/2/, 
http://kobalab.net/majiang/dapai.html etc.

## Roadmap
- South 4 Simulator
    - A game where you try to win in the last round [DONE]
- Hand analysis
    - Shanten for 13 tiles [DONE]
    - Shanten for 14 tiles with complete hand indication [DONE]
    - Ukeire, tile acceptance
    - Potential discards and their value    
    - Hand value for complete hand (14 tiles)
    - Hand value for tenpai hand with possible outcomes
    - All of the above also for hands with calls
- Table analysis
    - Safe tiles
    - Wait probability percentages
- Replay analysis
    - Tenhou replay parsing
    - Majsoul replay parsing
    - Discard rating
