# unc

A lightweight command-line HTTP client for testing APIs.

## Install

```
cargo build --release
```

The binary is written to `target/release/unc`.

## Usage

```
unc <METHOD> <URL> [-d <BODY>] [-H <HEADER>]...
```

- `METHOD` — `get`, `post`, `put`, or `delete` (case-insensitive)
- `URL` — must start with `http://` or `https://`
- `-d`, `--data` — JSON request body
- `-H`, `--header` — a `Name: Value` header, can be repeated

## Examples

```
unc get https://httpbin.org/get

unc post https://httpbin.org/post -d '{"title":"hello"}'

unc get https://httpbin.org/headers -H "Accept: application/json" -H "X-Test: abc"

unc delete https://httpbin.org/delete
```

## Output

Prints the response status (colored green/yellow/red), elapsed time, size,
and the body (pretty-printed if it's JSON).
