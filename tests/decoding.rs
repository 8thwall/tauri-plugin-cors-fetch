// Tests for decoding gzip, deflate, and brotli encoded data
// Run with: cargo test --test decoding

use flate2::{write::GzEncoder, write::DeflateEncoder, Compression};
use brotli::CompressorWriter;
use std::io::Write;

fn encode_gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn encode_deflate(data: &[u8]) -> Vec<u8> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn encode_brotli(data: &[u8]) -> Vec<u8> {
    let mut encoder = CompressorWriter::new(Vec::new(), 4096, 5, 22);
    encoder.write_all(data).unwrap();
    encoder.into_inner()
}

#[test]
fn test_gzip_decode() {
    let original = b"hello gzip";
    let encoded = encode_gzip(original);
    let mut decoder = flate2::read::GzDecoder::new(&encoded[..]);
    let mut decoded = Vec::new();
    std::io::Read::read_to_end(&mut decoder, &mut decoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_deflate_decode() {
    let original = b"hello deflate";
    let encoded = encode_deflate(original);
    let mut decoder = flate2::read::DeflateDecoder::new(&encoded[..]);
    let mut decoded = Vec::new();
    std::io::Read::read_to_end(&mut decoder, &mut decoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_brotli_decode() {
    let original = b"hello brotli";
    let encoded = encode_brotli(original);
    let mut decoder = brotli::Decompressor::new(&encoded[..], 4096);
    let mut decoded = Vec::new();
    std::io::Read::read_to_end(&mut decoder, &mut decoded).unwrap();
    assert_eq!(decoded, original);
}
