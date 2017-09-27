# RustFest 2017 - Workshop on AI & ML

In this repo are 3 games and 2 skeletons for writing bots.

As the first exercise, you should write a bot that plays the roguelike (or
bejeweled) game using a decision tree.

## Decision Tree Bot

First, go into the `roguelike` folder and `cargo run` and play a round (or few)
of the game. Try to see what what the minimum amount of moves to finish the game
is.

Switch to the `roguelike-tree-bot` folder and start building your bot. You
shouldn't expect to write a bot that plays the game perfectly on your first try,
getting it to even get to the goal will be an accomplishment in itself, then you
can try to optimize it further to make it play better.

* [Documentation for the roguelike game](https://folsen.github.io/rustfest2017/roguelike/index.html)
* [Documentation for `id_tree`](https://docs.rs/id_tree/1.1.3/id_tree/index.html)

## Learning Bot

For the learning game, the only game you will be able to play reasonably well
with a simple Q-Learning bot is the taxi game, where you are supposed to pick up
a passenger and deliver it to the goal (in the least amount of moves).

Again, you can go to the `taxi` folder and `cargo run` to play the game for
yourself first before trying to write a bot for it.

Most of the library-code (copied from the library examples) can be found in the
skeleton project under `taxi-learning-bot`, but it's up to you to write choose
the state representation and reward functions and details like that.

Once you have something working, you could try to optimize it to give it as small
state as possible, or try to train it in as few iterations as possible.

* [Documentation for the taxi game](https://folsen.github.io/rustfest2017/taxi/index.html)
* [Documentation for `renforce`](https://nivent.github.io/REnforce/renforce/)
