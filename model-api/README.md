# Model API

## About

A proof of concept for a web API that can export 3MF files from a parametric OpenSCAD model. A typical use would be to have a form on a website that allows users to enter the desired parameters, then submit the form to an API endpoint, allowing the user to download the generated 3MF file.


## Deployment

The API is a webserver written in Rust, running in a Docker container. I chose Docker, because it provided a way to also ship OpenSCAD in the same container.

If you don't want to run your own server, there are a lot providers that host Docker containers. I've been using [Clever Cloud](https://www.clever-cloud.com/) and I'm very happy with them.


## Usage

Build and run the development version:

```
cargo run
```

Build and run the production version:

```
docker build -t model-api .
docker run -p 80:80 model-api
```

Test API:
http://localhost/models/spacer.3mf?outer=30.0&inner=12.0&height=10.0
