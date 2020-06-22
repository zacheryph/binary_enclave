# Examples

All examples are setup pretty similarly. The below can be used for execution.
All examples use `serde_json` for templates as its produces the smallest code,
though there is zero reason you could not use toml, yaml or otherwise. All
examples should already contain an `example.json` to make playing with easier.

```
$ cargo build
$ ./target/debug/${example} -t > config.json
$ sha1sum ./target/debug/${example}
$ ./target/debug/${example} -w config.json
$ sha1sum ./target/debug/${example}
$ ./target/debug/${example} -p
```

## [`botpack`](botpack)

Where the idea for this fun code came from. just stores basic data that refers
to an irc botpack and the specific bots for a specific instance on a server.
