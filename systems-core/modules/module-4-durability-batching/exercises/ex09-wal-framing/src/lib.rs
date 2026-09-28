//! ex09 — Write-ahead log framing: length + checksum, and crash-safe replay.
//!
//! Record layout (little-endian):
//! ```text
//! [ len: u32 ][ crc32(payload): u32 ][ payload: len bytes ]
//! ```

pub const HEADER_LEN: usize = 8;
pub const MAX_RECORD: usize = 1 << 20; // 1 MiB

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalError {
    TooLarge(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replay {
    /// Every intact record, in order.
    pub records: Vec<Vec<u8>>,
    /// Bytes of the log that are valid. Truncate the file to this on recovery.
    pub valid_len: usize,
    /// True if bytes after `valid_len` exist (a torn or corrupt tail).
    pub torn_tail: bool,
}

/// CRC-32 (IEEE, reflected, polynomial 0xEDB88320).
pub fn crc32(data: &[u8]) -> u32 {
    // Coding map:
    // 1. crc = 0xFFFF_FFFF.
    // 2. For each byte: crc ^= byte; repeat 8 times:
    //      if crc & 1 == 1 { crc = (crc >> 1) ^ 0xEDB8_8320 } else { crc >>= 1 }
    // 3. Return !crc.
    todo!("bitwise CRC-32")
}

/// Frame one payload. Payloads over MAX_RECORD are rejected.
pub fn encode_record(payload: &[u8]) -> Result<Vec<u8>, WalError> {
    // Coding map:
    // 1. Reject payload.len() > MAX_RECORD with TooLarge(len).
    // 2. Allocate HEADER_LEN + len; push len as u32 LE, crc32 as u32 LE, payload.
    todo!("write the header then the payload")
}

/// Read records from the start of `log` until the end, or until the first
/// record that is incomplete, too large, or fails its checksum.
/// NEVER panics, whatever bytes it's given.
pub fn replay(log: &[u8]) -> Replay {
    // Coding map:
    // 1. pos = 0. While pos + HEADER_LEN <= log.len():
    //    a. Read len and crc with u32::from_le_bytes(slice.try_into().unwrap()).
    //    b. If len > MAX_RECORD -> stop (corrupt length).
    //    c. end = pos + HEADER_LEN + len; if end > log.len() -> stop (torn write).
    //    d. If crc32(payload) != crc -> stop (corruption).
    //    e. Push the payload, pos = end.
    // 2. valid_len = pos; torn_tail = pos < log.len().
    todo!("scan records until the first bad or partial one")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_of(records: &[&[u8]]) -> Vec<u8> {
        records.iter().flat_map(|r| encode_record(r).unwrap()).collect()
    }

    #[test]
    fn crc_known_vector() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn round_trip() {
        let log = log_of(&[b"a", b"", b"hello wal"]);
        let r = replay(&log);
        assert_eq!(r.records, vec![b"a".to_vec(), vec![], b"hello wal".to_vec()]);
        assert_eq!((r.valid_len, r.torn_tail), (log.len(), false));
    }

    #[test]
    fn torn_tail_is_cut_at_every_possible_crash_point() {
        let good = log_of(&[b"first", b"second"]);
        let third = encode_record(b"third record").unwrap();
        for cut in 0..third.len() {
            let mut log = good.clone();
            log.extend_from_slice(&third[..cut]);
            let r = replay(&log);
            assert_eq!(r.records.len(), 2, "cut at {cut}");
            assert_eq!(r.valid_len, good.len());
            assert_eq!(r.torn_tail, cut > 0);
        }
    }

    #[test]
    fn corruption_stops_replay_at_the_bad_record() {
        let mut log = log_of(&[b"one", b"two", b"three"]);
        let second_payload = HEADER_LEN + 3 + HEADER_LEN;
        log[second_payload] ^= 0xFF;
        let r = replay(&log);
        assert_eq!(r.records, vec![b"one".to_vec()]);
        assert!(r.torn_tail);
    }

    #[test]
    fn oversized_records() {
        assert_eq!(encode_record(&vec![0; MAX_RECORD + 1]), Err(WalError::TooLarge(MAX_RECORD + 1)));
        let mut log = (u32::MAX).to_le_bytes().to_vec();
        log.extend_from_slice(&[0; 12]);
        assert_eq!(replay(&log).valid_len, 0);
    }

    #[test]
    #[ignore = "learner must add a fuzz-style test"]
    fn learner_adversarial_test() {
        todo!("feed replay 10_000 pseudo-random byte strings and assert it never panics and valid_len <= len")
    }
}
