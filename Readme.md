# Speed comparaison for sha256

Libs used:
- [Sha2](https://docs.rs/sha2/latest/sha2/)
- [Ring](https://docs.rs/ring/latest/ring/)


## Results

| Test name                 |    Type    | Time per iteration  |  Error margins   |
|:--------------------------|:----------:|:-------------------:|:----------------:|
| test ring_1KB             | ... bench: |    2,700 ns/iter    |     (+/- 93)     |
| test rust_crypto_1KB      | ... bench: |    5,038 ns/iter    |    (+/- 126)     |
| test rust_crypto_ex_1KB   | ... bench: |    5,108 ns/iter    |     (+/- 89)     |
| test ring_10KB            | ... bench: |   21,876 ns/iter    |   (+/- 1,173)    |
| test rust_crypto_10KB     | ... bench: |   43,497 ns/iter    |    (+/- 953)     |
| test rust_crypto_ex_10KB  | ... bench: |   44,367 ns/iter    |    (+/- 789)     |
| test ring_100KB           | ... bench: |   222,529 ns/iter   |   (+/- 15,291)   |
| test rust_crypto_100KB    | ... bench: |   430,956 ns/iter   |   (+/- 15,981)   |
| test rust_crypto_ex_100KB | ... bench: |   436,233 ns/iter   |   (+/- 10,595)   |
| test ring_1MB             | ... bench: |  2,208,477 ns/iter  |  (+/- 112,062)   |
| test rust_crypto_1MB      | ... bench: |  4,326,755 ns/iter  |   (+/- 64,751)   |
| test rust_crypto_ex_1MB   | ... bench: |  4,532,017 ns/iter  |  (+/- 196,418)   |
| test ring_10MB            | ... bench: | 22,540,317 ns/iter  | (+/- 1,031,470)  |
| test rust_crypto_10MB     | ... bench: | 43,503,515 ns/iter  |  (+/- 365,473)   |
| test rust_crypto_ex_10MB  | ... bench: | 44,785,722 ns/iter  |  (+/- 381,125)   |
| test ring_100MB           | ... bench: | 221,364,855 ns/iter | (+/- 7,963,665)  |
| test rust_crypto_100MB    | ... bench: | 437,011,342 ns/iter | (+/- 15,169,006) |
| test rust_crypto_ex_100MB | ... bench: | 440,839,536 ns/iter | (+/- 4,325,270)  |

## Tests
All function are run in a loop with an unique setup as :
```rust
let input = std::fs::File::open($path)?;
let mut reader = std::io::BufReader::new(input);
```

Then at each end of a loop an unique tear down is run :
```rust
std::io::Seek::seek(&mut reader, std::io::SeekFrom::Start(0))
```

Those codes are run in the loops

### Sha2 (RustCrypto)

#### Simple version using streamable interface (rust_crypto)
```rust
let mut sha256: sha2::Sha256 = sha2::Digest::new(); 
std::io::copy(&mut reader, &mut sha256)?; 
let r = test::black_box(sha2::Digest::finalize(sha256)); 
assert_eq!(r.as_ref(), $result);
```

#### Manual version using a loop and buffer (rust_crypto_ex)
```rust
let mut sha256: sha2::Sha256 = sha2::Digest::new();
let mut buffer = [0; 1024];
loop {
    let count = std::io::Read::read(&mut reader, &mut buffer)?;
    if count == 0 {
        break;
    }
    sha2::Digest::update(&mut sha256, &buffer[..count]);
}
let r = test::black_box(sha2::Digest::finalize(sha256));
assert_eq!(r.as_ref(), $result);
```

### Ring

#### Simple version using context and manual loop (ring)
```rust
let mut context = ring::digest::Context::new(&ring::digest::SHA256);
let mut buffer = [0; 1024];
loop {
    let count = std::io::Read::read(&mut reader, &mut buffer)?;
    if count == 0 {
        break;
    }
    context.update(&buffer[..count]);
}
let r=test::black_box(context.finish());
assert_eq!(r.as_ref(), $result);
```

## File generation
```shell
rm -rf files && mkdir files
dd bs=1024 count=1 </dev/urandom > files/1KB
dd bs=1024 count=10 </dev/urandom > files/10KB
dd bs=1024 count=100 </dev/urandom > files/100KB
dd bs=1024 count=1000 </dev/urandom > files/10MB
dd bs=1024 count=10000 </dev/urandom > files/10MB
dd bs=1024 count=100000 </dev/urandom > files/100MB
```

## Computer specs
Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz