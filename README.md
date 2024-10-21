# Codecrafters - Build your own HTTP server

Solution to the [Build your own HTTP server](https://app.codecrafters.io/courses/http-server/overview) project.

Run locally:

```
cargo run -- --directory /tmp/
```

Test via `curl`:
```
curl -v --data "Hello! From file 234" -H "Content-Type: application/octet-stream" http://localhost:4221/files/file_234
```

TODO:

- [x] code clean up
- [x] add proper error handling
- [ ] tests - previously tested via codecrafters service
- [ ] add cli front end
