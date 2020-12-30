# Learning Rust

Just some stuff while learning some rust.. its probably terrible.

### Goal

Write an AWS lambda function that can take a request, do some routing, call some other services, and return some content.

### Progress

- bin/basic.rs - print out the json that gets sent as an API gateway request
- bin/hello.rs - return a message in the right format for the API gateway
- bin/convert.rs - convert api gateway request to http::Request which seems to be a thing

### Building

Need:
 - rust
 - sam

```
 sam --debug build
 sam local start-api
```

in another shell
```
 curl -XPOST http://127.0.0.1:3000/bob
```

