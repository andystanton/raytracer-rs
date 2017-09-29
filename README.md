# A multithreaded Ray Tracer in Rust

![](https://andystanton.github.io/raytracer-rs/raytracing-random-scene@320x240.png)

A multithreaded Rust implementation of the ray tracer from Peter Shirley's [Ray Tracing in One Weekend](https://www.amazon.com/gp/product/B01B5AODD8).

## Usage

The raytracer is a command line application that writes an image of a given scene to disk with specific parameters, and outputs the path of the image.

```sh
$ cargo run --release -- \
    --scene random \
    --nx 300 \
    --ny 200 \
    --samples-per-pixel 50
```

For a full list of commands and scenes, run:

```sh
$ cargo run --release -- --help
```

## Screenshots

[![Random Scene](https://andystanton.github.io/raytracer-rs/raytracing-random-scene@320x240.png)](https://andystanton.github.io/raytracer-rs/raytracing-random-scene@1024x768.png)
[![Teapot Scene](https://andystanton.github.io/raytracer-rs/raytracing-teapot-scene@320x240.png)](https://andystanton.github.io/raytracer-rs/raytracing-teapot-scene@1024x768.png)
[![Another Random Scene](https://andystanton.github.io/raytracer-rs/raytracing-random2-scene@320x240.png)](https://andystanton.github.io/raytracer-rs/raytracing-random2-scene@1024x768.png)
[![Test Scene](https://andystanton.github.io/raytracer-rs/raytracing-test-scene@320x240.png)](https://andystanton.github.io/raytracer-rs/raytracing-test-scene@1024x768.png)
