# Codecrafters - Build your own HTTP server

My solution to the [Build your own HTTP server](https://app.codecrafters.io/courses/http-server/overview) project.

Run locally:

```
cargo run -- --directory /tmp/
```

Test via `curl`:
```
curl -v --data "Hello! From file 234" -H "Content-Type: application/octet-stream" http://localhost:4221/files/file_234
```

TODO:

- [ ] handle subdir creation issue and add test
- [ ] remove redundant option in Mutex
- [ ] update `README` with testing `curl`s
- [ ] thread pool?
