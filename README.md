# papi-wrap

Rust bindings for the PAPI (Performance API) library.

This project is a modernized fork of:
https://github.com/eholk/rust-papi

## System dependency

This crate links to the native `papi` C library. You must install PAPI on your
system before building this crate.

On Debian/Ubuntu:

```bash
sudo apt-get update
sudo apt-get install -y libpapi-dev
```

## Add to your project

```toml
[dependencies]
papi-wrap = "0.5"
```

## Quick start

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
	papi_wrap::initialize(true)?;
	// create and use counters/events sets here
	papi_wrap::terminate();
	Ok(())
}
```

## Notes

- Documentation: https://docs.rs/papi-wrap
- Repository: https://github.com/A924404/papi-bindings-rs
- Upstream PAPI project: http://icl.cs.utk.edu/papi/
