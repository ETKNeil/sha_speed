#![allow(non_snake_case)]
#![feature(test)]
extern crate test;

static FIXED_1KB: [u8; 32] = [113, 67, 68, 127, 89, 23, 10, 91, 36, 248, 225, 122, 105, 236, 188, 222, 56, 163, 192, 83, 168, 107, 12, 78, 13, 21, 4, 13, 148, 183, 126, 119];
static FIXED_10KB: [u8; 32] = [162, 75, 91, 197, 0, 50, 74, 153, 110, 94, 109, 143, 40, 208, 2, 236, 162, 85, 138, 72, 38, 255, 28, 153, 55, 177, 16, 85, 58, 89, 147, 173];
static FIXED_100KB: [u8; 32] = [173, 223, 145, 207, 81, 162, 12, 143, 250, 10, 186, 64, 235, 28, 29, 44, 215, 85, 77, 225, 218, 194, 47, 82, 240, 200, 142, 144, 62, 58, 245, 160];
static FIXED_1MB: [u8; 32] = [165, 172, 62, 132, 119, 234, 215, 194, 230, 52, 245, 247, 196, 117, 195, 69, 95, 240, 139, 7, 202, 204, 187, 125, 102, 156, 3, 251, 238, 14, 227, 185];
#[cfg(feature = "large_files")]
static FIXED_10MB: [u8; 32] = [73, 183, 38, 116, 67, 147, 134, 117, 123, 60, 226, 250, 234, 37, 65, 243, 191, 179, 68, 51, 190, 161, 217, 166, 214, 217, 90, 40, 12, 62, 247, 140];
#[cfg(feature = "large_files")]
static FIXED_100MB: [u8; 32] = [140, 84, 202, 12, 239, 36, 127, 41, 219, 44, 15, 104, 146, 67, 255, 37, 123, 109, 229, 49, 184, 96, 133, 91, 65, 164, 245, 112, 208, 166, 104, 10];


macro_rules! rust_crypto {
    ($func:ident, $path:expr, $result:expr) => {
        #[bench]
        fn $func(b: &mut test::Bencher) -> anyhow::Result<()> {
            let input = std::fs::File::open($path)?;
            let mut reader = std::io::BufReader::new(input);
            b.iter(|| {
                let mut sha256: sha2::Sha256 = sha2::Digest::new();
                std::io::copy(&mut reader, &mut sha256)?;
                let r = test::black_box(sha2::Digest::finalize(sha256));
                assert_eq!(r.as_ref(), $result);
                std::io::Seek::seek(&mut reader, std::io::SeekFrom::Start(0))
            });
            Ok(())
        }
    };
}

rust_crypto!(rust_crypto_1KB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "1KB"),FIXED_1KB);
rust_crypto!(rust_crypto_10KB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "10KB"),FIXED_10KB);
rust_crypto!(rust_crypto_100KB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "100KB"),FIXED_100KB);
rust_crypto!(rust_crypto_1MB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "1MB"),FIXED_1MB);
#[cfg(feature = "large_files")]
rust_crypto!(rust_crypto_10MB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "10MB"),FIXED_10MB);
#[cfg(feature = "large_files")]
rust_crypto!(rust_crypto_100MB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "100MB"),FIXED_100MB);

macro_rules! rust_crypto_ex {
    ($func:ident, $path:expr, $result:expr) => {
        #[bench]
        fn $func(b: &mut test::Bencher) -> anyhow::Result<()> {
            let input = std::fs::File::open($path)?;
            let mut reader = std::io::BufReader::new(input);
            b.iter(|| {
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
                std::io::Seek::seek(&mut reader, std::io::SeekFrom::Start(0))
            });
            Ok(())
        }
    };
}

#[test]
fn test(){
    sha2::Sha256
}

rust_crypto_ex!(rust_crypto_ex_1KB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "1KB"),FIXED_1KB);
rust_crypto_ex!(rust_crypto_ex_10KB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "10KB"),FIXED_10KB);
rust_crypto_ex!(rust_crypto_ex_100KB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "100KB"),FIXED_100KB);
rust_crypto_ex!(rust_crypto_ex_1MB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "1MB"),FIXED_1MB);
#[cfg(feature = "large_files")]
rust_crypto_ex!(rust_crypto_ex_10MB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "10MB"),FIXED_10MB);
#[cfg(feature = "large_files")]
rust_crypto_ex!(rust_crypto_ex_100MB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "100MB"),FIXED_100MB);

macro_rules! ring {
    ($func:ident, $path:expr, $result:expr) => {
        #[bench]
        fn $func(b: &mut test::Bencher) -> anyhow::Result<()> {
            let input = std::fs::File::open($path)?;
            let mut reader = std::io::BufReader::new(input);
            b.iter(|| {
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
                std::io::Seek::seek(&mut reader, std::io::SeekFrom::Start(0))
            });
            Ok(())
        }
    };
}

ring!(ring_1KB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "1KB"),FIXED_1KB);
ring!(ring_10KB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "10KB"),FIXED_10KB);
ring!(ring_100KB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "100KB"),FIXED_100KB);
ring!(ring_1MB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "1MB"),FIXED_1MB);
#[cfg(feature = "large_files")]
ring!(ring_10MB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "10MB"),FIXED_10MB);
#[cfg(feature = "large_files")]
ring!(ring_100MB,concat!(env!("CARGO_MANIFEST_DIR"), "/files/", "100MB"),FIXED_100MB);
