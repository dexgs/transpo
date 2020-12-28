# transpo
A simple file sharing web application I hacked together in just under 4 days. 
Since it's been completed I am slowly tidying and improving it.

<b><s>A complete ripoff of</s></b> Inspired by the late [Firefox Send](https://github.com/mozilla/send) which I miss very dearly.

![screenshot of frontend](screenshot.png)

## Features:
* Upload files that will last for a given amount of time
* Option to limit the number of times a file can be downloaded before it expires
* Option to password protect files
* Compresses large and multi-file uploads

## Planned:
* Client-side encryption and decryption of files

## Installation:
Clone this repository, adjust the settings in `src/config.rs` to your liking, navigate to the directory and build it with `cargo build --release` and run it with `cargo run --release`
