use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fs::File;
use std::io::{self, Read};

const HASH_SIZE: usize = 32;
const LINE_LENGTH: u64 = 65;

#[rustler::nif]
fn root(file_path: String) -> String {
    let (mut data, count) = read_data(&file_path).expect("Data should be readable and valid");
    do_root(&mut data, count)
}

/// Computes the merkle root from the bytes vector.
///
/// The `data` contains consecutive hash entries. The algorithm takes one pair of hash entries from
/// `data`, hashes it and stores the result as a first entry of `data`. Then it takes a next pair of hash
/// entries and stores their hash as a second entry of `data`. After all entries are exhausted,
/// it starts over with the newly created entries. When there is only a single entry left, it is
/// the merkle root and it is returned as a hex-encoded string.
///
/// When there is an even number of entries at any time of the computation, the last one is used as
/// is for the next round of computation. There are more ways how to address this even-
///
/// The computation iterations look like this:
///
/// |      1 |  2 |  3 |  4 |  5 |  6 |
///
/// |     12 | 34 | 56 |
///
/// |   1234 | 56 |
///
/// | 123456 |
///
/// The same vector is reused for performance reasons, so it is not necessary to allocate a new one
/// in each iteration. The number of current entries is tracked separately - the tail of the vector
/// is not cleared.
fn do_root(data: &mut Vec<u8>, mut count: u64) -> String {
    let mut hasher = Sha256::new();
    let mut from_idx: usize;
    let mut to_idx: usize;

    while count > 1 {
        from_idx = 0;
        to_idx = 0;

        while (from_idx as u64) < count {
            // If the entry count is even and this is the last one, just use it as it is.
            // Otherwise hash the pair of entries.
            if from_idx as u64 + 1 == count {
                copy_hash(data, from_idx, to_idx);
            } else {
                hash_pair(&mut hasher, data, from_idx, to_idx);
            }
            from_idx += 2;
            to_idx += 1;
        }
        // Next count is a half of the current count. In case there was even number of records, it
        // is rounded up.
        count = (count + 1) / 2;
    }
    hex::encode(&data[0..HASH_SIZE])
}

/// Copies a hash entry inside the bytes vector.
///
/// The entry is taken at the `from` index of `data` and it is written at the `to` index of `data`.
fn copy_hash(data: &mut Vec<u8>, from: usize, to: usize) {
    let (dst, src) = data.split_at_mut(from * HASH_SIZE);
    dst[(to * HASH_SIZE)..((to + 1) * HASH_SIZE)]
        .clone_from_slice(&src[..HASH_SIZE]);
}

/// Hashes a consecutive pair of entries.
///
/// The hash pair is taken at the `from` index of `data` and the resulting hash is written at the
/// `to` index of `data`.
fn hash_pair(hasher: &mut Sha256, data: &mut Vec<u8>, from: usize, to: usize) {
    hasher.reset();
    hasher.input(&data[(from * HASH_SIZE)..((from + 2) * HASH_SIZE)]);
    hasher.result(&mut data[(to * HASH_SIZE)..((to + 1) * HASH_SIZE)]);
}

/// Reads data from file to a bytes vector.
///
/// Each line in the file is expected to consist of 64 hexadecimal characters, each representing a
/// transaction id (a hash).
///
/// The resulting vector is a continuous sequence of bytes decoded from the hex-encoded lines.
/// Each resulting hash entry has 32 bytes. There is no separator between the entries, it is
/// up to caller to address them via indexes.
fn read_data(file_path: &str) -> io::Result<(Vec<u8>, u64)> {
    // Read the whole file into string.
    let mut file = File::open(file_path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    // Allocate vector for all the data converted to bytes.
    let count = line_count(&file)?;
    let mut data: Vec<u8> = Vec::new();
    data.resize(count as usize * HASH_SIZE, 0u8);

    // Convert hex-encoded lines into bytes.
    for (index, line) in s.lines().enumerate() {
        let start = index * HASH_SIZE;
        let end = (index + 1) * HASH_SIZE;
        hex::decode_to_slice(line, &mut data[start..end])
            .expect("Line should consist of 64 hexadecimal characters");
    }

    Ok((data, count as u64))
}

/// Counts the number of lines in the file.
///
/// Each line in the file is expected to be 65 bytes long (64 bytes hex-string + '\n').
/// Alternative line endings (such as '\r\n') are not anticipated.
fn line_count(file: &File) -> io::Result<u64> {
    let len = file.metadata()?.len();

    // Anticipate there can be no '\n' at the end of the file.
    let count = if len % LINE_LENGTH == 0 {
        len / LINE_LENGTH
    } else {
        len / LINE_LENGTH + 1
    };

    Ok(count)
}

rustler::init!("Elixir.MerkleTreeRust", [root]);
