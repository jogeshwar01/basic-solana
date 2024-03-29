// Release profiles - dev, test, release - default is dev
// cargo build/run --release - to build/run in release mode
// can specify profile in Cargo.toml with opt-levels. 0 being the lowest and 3 being the highest. 
// 0 is the default for dev and 3 is the default for release. 0 has faster compile times and 3 has faster runtime.

// to set opt-level in Cargo.toml:
// [profile.dev]
// opt-level = 0

// [profile.release]
// opt-level = 3

// to set a custom profile:
// [profile.custom]
// opt-level = 3

// to use the custom profile:
// cargo build --profile custom
// cargo run --profile custom
// cargo test --profile custom
