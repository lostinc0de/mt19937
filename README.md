# prng_mt19937
Implementation of the Mersenne Twister PRNG in Rust

This is a simple implementation of the Mersenne-Twister pseudorandom number generator in Rust using the algorithm on Wikipedia.
The macro mersenne_twister() is provided to create the structs MT19937 and MT19937_64 for the 32 bit and 64 bit versions respectively.
Different MT PRNGs may be constructed with non-default parameters using this macro.
The implementation has been tested against the C++ PRNG std::mt19937.
To generate pseudorandom numbers instantiate the object with a seed using new() and call next():

```rust
    use prng_mt::mt19937::MT19937;
    let mut mt = MT19937::new(42);
    println!("Rand: {}", mt.next());
```
