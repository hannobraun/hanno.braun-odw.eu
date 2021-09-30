# Model API

## Usage

Build and run development version:

```
cargo run
```

Build and run production version:

```
docker build -t model-api .
docker run -p 80:80 model-api
```

Test API:
http://localhost/models/spacer.3mf?outer=30.0&inner=12.0&height=10.0
