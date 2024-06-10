# Pingpong Calculation

A calculation of pingpong play sport using go and rust.

## Apis

- `GET /ping`: Check the server status.

  - Parameters: None
  - Example: `GET /ping`
  - Response: `message: "pong"`

- `GET /best_action`: Get the probobility of next action result calculation of the given action.

  - Parameters: `action` (string)
  - Example: `GET /best_action?action=SB_2`
  - Response: `message: { "best_action": "E/N_1", "best_score": 0.25, "action_scores": [...] }`

## Local Development

Environment requirements:

- [Rust](https://www.rust-lang.org/tools/install): 1.78.0+
- [Go](https://golang.org/doc/install): 1.16.0+

Clone the repository:

```shell
git clone https://github.com/cworld1/pingpong-calculation.git
```

Make the project:

```shell
cd pingpong-calculation
make run-dynamic
# Or run: make run-static
```

## Contributions

To spend more time coding and less time fiddling with whitespace, this project uses code conventions and styles to encourage consistency. Code with a consistent style is easier (and less error-prone!) to review, maintain, and understand.

## Thanks

- [Rust + Go(lang)](https://github.com/mediremi/rust-plus-golang)
- [rustgo: Calling Rust from Go with near-zero overhead](https://words.filippo.io/rustgo/)
- [Hooking Go from Rust](https://metalbear.co/blog/hooking-go-from-rust-hitchhikers-guide-to-the-go-laxy/)

## License

This project is licensed under the GPL 3.0 License.
