# Bevy lab - private learning project

The only purpose of making this project public is to document my Rust + Bevy journey.
The README file is organized in a journal-style format, where I write something about each developed app.

My goal is to make simple projects and not focus on refactoring for now. I'll try to not use AI agents.

Throughout this journey, my main source of information about the engine will be https://taintedcoders.com/.

I'll build all games for windows and tag as a release.

## 1. Pong

A custom version of pong that is meant for 2 players.
Paddles are controlled using [W/S] and [ArrowUp/ArrowDown].

That was a great idea to start so small. I learned a lot, mainly that the whole state management isn't that easy
and testing for different scenarios is really importatnt.

#### Problems met during development:

- avian2d compound collider is not working with collider ellipse (https://github.com/avianphysics/avian/issues/369)
- AI agents couldn't help me with asset tracking and making an asset a dependency (there was a macro for that)

## 2. Snake

A traditional snake with simple sounds. It lacks menus and winning mechanism.