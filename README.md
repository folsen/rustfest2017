# The Bejeweled Challenge

If you want to play a command-line version of Bejeweled, clone this repo, install rust and run `cargo run` from the root of the directory.

A game board will be displayed and you will be prompted for input. The move syntax is `row1 col1 row2 col2` with spaces separating the numbers.

The grid is top-left to bottom-right oriented. So `0 0` is the top left corner and `7 7` is the bottom right corner.

Example:
```
-----------------
0               0
-----------------
|G|G|O|R|G|G|R|G|
|R|W|O|R|G|W|B|R|
|R|P|G|G|P|P|B|W|
|G|W|P|O|G|W|W|G|
|B|G|G|B|G|G|R|G|
|B|O|G|R|O|O|W|R|
|R|B|O|O|G|P|B|G|
|O|W|W|G|W|B|W|O|
-----------------
2 2 3 2
-----------------
1              10
-----------------
|G|G|P|R|G|G|R|G|
|R|W|G|R|G|W|B|R|
|R|P|R|G|P|P|B|W|
|G|W|O|O|G|W|W|G|
|B|G|O|B|G|G|R|G|
|B|O|P|R|O|O|W|R|
|R|B|O|O|G|P|B|G|
|O|W|W|G|W|B|W|O|
-----------------
```
