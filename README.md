# Rust HTTP Server

## Overview

A simple http server implemented in rust.

+ Maintain / close connection, handle multiple http request from single or multiple TCP connection.

+ Support to read files, write files at server end and return file contents.
+ Support to compress response body with gzip.

## Dependency

+ **threadpool**: Handle http request concurrently.
+ **flate2**: Compress http response body.

## License

+ source: [Build your own HTTP server | CodeCrafters](https://app.codecrafters.io/courses/http-server/overview)

