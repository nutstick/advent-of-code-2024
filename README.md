# 🎄 Advent of Code 2024 in Rust

My [Advent of Code 2024](https://adventofcode.com/2024) solutions in the Rust programming language. This package using [Advent of Code Runner](https://github.com/gobanos/cargo-aoc/tree/v0.3/aoc-runner)

### Setting up the CLI

You will need to find your session token for the AoC in order for cargo-aoc to work. Thankfully, finding your token is easy since it is stored in your Browser's cookies. Open up the devtools of your browser, and then :

Firefox: "Storage" tab, Cookies, and copy the "Value" field of the session cookie.
Google Chrome / Chromium: "Application" tab, Cookies, and copy the "Value" field of the session cookie.
Once you have it, simply run : `cargo aoc credentials {token}`

You're now ready to start coding !

NOTE: If for some reason your token has changed, dont forget to change it back.

`cargo aoc credentials` will show the currently stored user token

## Downloading your input manually

`cargo aoc input` will download an input and store it in `input/{year}/day{day}.txt.`

Please note that by default, we're taking today's date as the argument. Of course, you can change this using :

```sh
# cargo aoc input -d {day} -y {year}
cargo aoc input -d 1 -y 2024
```

## Running the solution

`cargo aoc` will run the latest implemented day, downloading your input beforehand. It will show you the result, and a short summary of how well it did perform.

```sh
cargo aoc
# cargo aoc -d {day} -p {part}
cargo aoc -d 1 -p 1
```
