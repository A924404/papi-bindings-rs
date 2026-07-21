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
	Ok(())
}
```

## Common Issues
### Counters return 0s
If your counters return 0s, check the `perf_event_paranoid` value. Make sure it is 0 or less:

```bash
sysctl kernel.perf_event_paranoid
sudo sysctl -w kernel.perf_event_paranoid=-1
```

### It cannot find the PAPI library
If you installed the latest PAPI library from source, make sure its location is in your library path:

```bash
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH
```

### How can I know if PAPI works?
After installing PAPI, run the following command to check whether it recognizes your CPU/GPU:

```bash
papi_component_avail
```

To check which PAPI events are available on your machine, run:

```bash
papi_avail
```

## Notes

- Documentation: https://docs.rs/papi-wrap
- Upstream PAPI project: https://github.com/icl-utk-edu/papi
