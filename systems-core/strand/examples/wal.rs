//! A tiny write-ahead log (WAL) with group commit.
//! Run: cargo run --release --example wal
//!
//! Design (the same shape real databases use):
//! * ONE writer thread owns the file. No lock around the file at all:
//!   everyone else talks to it through a bounded channel.
//! * Each append carries a reply channel ("oneshot"), so the client learns
//!   its log position (LSN) only AFTER the record is safely on disk.
//! * Group commit: the writer grabs every request waiting in the queue,
//!   writes them all, then does ONE fsync. fsync is slow (~ms), so batching
//!   it is the difference between 500 and 50,000 writes/sec.
//! * Each record is framed [len u32][crc32 u32][payload]. On restart we
//!   replay until the first bad/partial record: a crash mid-write leaves a
//!   "torn" tail that the checksum catches, and we truncate it.
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use strand::*;

struct Append {
    data: Vec<u8>,
    ack: Sender<u64>, // where to send the LSN once durable
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

fn frame(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + data.len());
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(&crc32(data).to_le_bytes());
    out.extend_from_slice(data);
    out
}

/// Returns the valid records and how many bytes of the file are valid.
fn replay(bytes: &[u8]) -> (Vec<Vec<u8>>, usize) {
    let (mut pos, mut records) = (0, Vec::new());
    while pos + 8 <= bytes.len() {
        let len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        let crc = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap());
        let end = pos + 8 + len;
        if end > bytes.len() || crc32(&bytes[pos + 8..end]) != crc {
            break; // torn or corrupt tail: stop here
        }
        records.push(bytes[pos + 8..end].to_vec());
        pos = end;
    }
    (records, pos)
}

fn main() -> std::io::Result<()> {
    let path = std::env::temp_dir().join("strand-demo.wal");
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&path)?;

    chan!(wal: Append, bounded 64);

    // 8 clients, 250 appends each. Each waits for its own ack.
    let clients = spawn!(8, [wal], |client| {
        let mut last_lsn = 0;
        for i in 0..250 {
            chan!(ack: u64); // a one-shot reply channel
            send!(wal, Append { data: format!("client {client} op {i}").into_bytes(), ack: ack.sender() });
            last_lsn = ack.into_receiver().recv().expect("writer died");
        }
        last_lsn
    });

    // Main thread = the single writer.
    let rx = wal.into_receiver(); // drops our Sender: loop ends when clients finish
    let (mut lsn, mut batches, mut largest) = (0u64, 0, 0);
    while let Ok(first) = rx.recv() {
        let mut batch = vec![first];
        batch.extend(rx.drain_ready()); // everything else already waiting
        for req in &batch {
            file.write_all(&frame(&req.data))?;
        }
        file.sync_data()?; // ONE fsync for the whole batch = group commit
        for req in batch.iter() {
            lsn += 1;
            let _ = req.ack.send(lsn); // durable now: tell the client
        }
        batches += 1;
        largest = largest.max(batch.len());
    }
    let last_per_client = clients.wait();
    println!("wrote {lsn} records in {batches} fsyncs (largest batch {largest})");
    println!("last LSN seen by each client: {last_per_client:?}");

    // Simulate a crash in the middle of a write: half a record at the end.
    file.write_all(&frame(b"this write never finished")[..10])?;
    drop(file);

    // Recovery.
    let mut bytes = Vec::new();
    File::open(&path)?.read_to_end(&mut bytes)?;
    let (records, valid) = replay(&bytes);
    println!(
        "replay: {} records valid, {} torn bytes at the tail -> truncating",
        records.len(),
        bytes.len() - valid
    );
    OpenOptions::new().write(true).open(&path)?.set_len(valid as u64)?;
    assert_eq!(records.len() as u64, lsn);
    println!("first record: {:?}", String::from_utf8_lossy(&records[0]));
    Ok(())
}
