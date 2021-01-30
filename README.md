# transpo
A simple file sharing web application that is <b><s>a total ripoff of</s></b> inspired by the late 
[Firefox Send](https://github.com/mozilla/send) which I miss very dearly.

![screenshot of frontend](screenshot.png)

## Features:
* Upload files that will last for a given amount of time
* Option to limit the number of times a file can be downloaded before it expires
* Uses the IndexedDB API to commit large files to disk during download (saves memory)
* Option to password protect files
* Compresses multi-file uploads
* Client-side encryption and decryption of files (not bullet-proof, but a whole lot better than nothing)
* Themes (light and dark are inculded in this repository, but adding your own is trivial)

## Notes:
* IndexedDB does not currently download the file from storage on Firefox (tested on 84)

## TO-DO:
* Better error messages for the client
* Clean up my JavaScript (this was my first real JS project)

## Installation and Setup:
1) Make sure you have cargo installed.
2) Clone this repository and navigate to its directory.
3) Adjust the settings in `src/config.rs` to your liking.
4) Build with `cargo build --release` and run with `cargo run --release`.
5) After getting it to work, you should probably set up your OS's init system to run transpo as a service.

Example Nginx config for transpo running on port 8080:
```nginx
location /transpo/ {
  client_max_body_size 500M;
  proxy_pass http://127.0.0.1:8080/;
  proxy_http_version 1.1;
  proxy_set_header Upgrade $http_upgrade;
  proxy_set_header Connection "upgrade";
}

map $http_upgrade {
        default upgrade;
        '' close;
}
```
