# Weather CLI app
This application was created as a test task to join Polytech Software.

## General
In this application I implemented two weather providers, namely OpenWeather and WeatherApi.
One can choose to get weather from either, but the output will be slightly different because
of provider-related differences in weather information.
The provider configuration file is saved to `$CONFIG_DIR/weatherapp/provider`.

## Note on "production readiness"
This note mainly concerns API keys. Since the API keys must be read from a local file,
it cannot be considered "production-ready", since it will not be installable using
`cargo install`. On the other hand, I choose not to implement platform-specific builds
for Docker image or binary distribution because of the API key reason. If I really had to build
a minimal, but production-ready app, it would require at least some "proxy" infrastructure,
where the requests would be made to some proxy server that can access the API keys (hosted
in some service like Redis/Kong, or cloud primitives, like k8s secrets), which would in turn forward
the requests to real providers with the API keys added.

## Run instructions
There are two ways to run the project. First approach is to build the project from source
with api keys provided in [`apikeys.conf`](apikeys.conf). **NOTE**: one needs to create
the file using the syntax in [`apikeys.example.conf`](apikeys.example.conf).
Since the app uses cross-platform libraries, it easily compiles to all targets (Windows, OSX, Linux).

### Compiled from source
To get the weather, simply invoke:
```shell
cargo run --release -- get <LOCATION>
```
If the location consists more than two words, please encase them in double quotes ("").

To change the provider, simply invoke:
```shell
cargo run --release -- configure (openweather|weatherapi)
```

### Using pre-built binary
To get the weather, simply invoke:
```shell
weather get <LOCATION>
```
If the location consists more than two words, please encase them in double quotes ("").

To change the provider, simply invoke:
```shell
weather configure (openweather|weatherapi)
```