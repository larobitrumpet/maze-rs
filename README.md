# Rust Maze Generation

Maze generation in Rust! This is similar to my other [maze generation project](https://github.com/larobitrumpet/maze), except this is written in rust instead of C. This is my first attempt at a project written in Rust.

## Dependencies

This project is written in Rust and thus requires the Rust compiler, which you can find [here](https://www.rust-lang.org/learn/get-started).

This project uses a Rust wrapper and bindings for the Allegro 5 game library provided by [SiegeLord](https://github.com/SiegeLord). Instructions for installing Allegro 5 can be found [here](https://github.com/liballeg/allegro_wiki/wiki/Quickstart). You should also look at [RustAllegro](https://github.com/SiegeLord/RustAllegro), especially if you are using Windows.

## Build

Once you have Rust and Allegro 5 installed, all you need to do is run `cargo build` to build a debug version, and `cargo build --release` for a release version.

## Run

To run, just run `cargo run` or `cargo run --release`.

## Usage

You will be asked the dimensions of the maze and what algorithm to use. To choose an algorithm, simply enter the number corresponding to that algorithm.
If you choose the Growing Tree algorithm, you will also be asked to provide weights for the different methods of choosing a cell. For example, if you want the algorithm to choose the newest cell added 50% of the time and choose a random cell 50% of the time, then you would give newest and random weights of 1, and all other methods weights of 0:
```
  Newest: 1
  Random: 1
  Middle: 0
  Oldest: 0
```
If you want to choose the newest cell added 75% of the time, a random cell 25% of the time, you would give newest a weight of 3 and random a weight of 1:
```
  Newest: 3
  Random: 1
  Middle: 0
  Oldest: 0
```
If you only want to choose the newest cell added, then give newest a weight of 1 and everything else a weight of 0:
```
  Newest: 1
  Random: 0
  Middle: 0
  Oldest: 0
```
Choosing only the newest is equivelent to the Recursive Backtracking algorithm and choosing only randomly is equivelent to Prim's algorithm.

## License

This application is licensed under the [MIT License](LICENSE).
