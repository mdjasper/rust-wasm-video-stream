# rust-wasm-video-stream

This is a personal learning side project investigating rust and wasm. It reads
a video stream from a browser, modifies each frame by plucking out a certain
color, then sends the stream to a video element in the DOM.

![black and white video stream with a blue object visible][screenshot.jpg?raw=true]

# build

run `build.sh` to build the rust dependencies and run webpack dev server

run `build-wasm.sh` to specifically build rust while webpack is running

the app will be running on `localhost:8080`
