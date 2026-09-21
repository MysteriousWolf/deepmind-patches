//! Enough MP3 parsing to enforce the demo limits.
//!
//! Walks the frame headers, skipping an `ID3v2` tag at the front and an `ID3v1`
//! tag at the back. Reports duration, bitrate, sample rate and channels. Does
//! not decode audio.

use std::path::Path;

use crate::error::{Error, Result};

/// The demo format the repository accepts.
pub mod limits {
    /// Bitrate, kbit/s, constant.
    pub const BITRATE_KBPS: u32 = 128;
    /// Sample rate, Hz.
    pub const SAMPLE_RATE: u32 = 44_100;
    /// Longest a demo may run, seconds.
    pub const MAX_SECONDS: f64 = 20.0;
    /// Largest a demo file may be, bytes.
    pub const MAX_BYTES: u64 = 350 * 1024;
}

/// What the frame headers of a file say.
#[derive(Debug, Clone, PartialEq)]
pub struct Mp3Info {
    /// Playing time in seconds.
    pub seconds: f64,
    /// Bitrate of the first frame, kbit/s.
    pub bitrate_kbps: u32,
    /// Whether every frame has the same bitrate.
    pub constant_bitrate: bool,
    /// Sample rate, Hz.
    pub sample_rate: u32,
    /// 1 or 2.
    pub channels: u8,
    /// Frames counted.
    pub frames: u32,
}

impl Mp3Info {
    /// Reads the frame headers of a file's bytes.
    pub fn parse(path: &Path, bytes: &[u8]) -> Result<Self> {
        let fail = |message: &str| Error::Mp3 {
            path: path.to_owned(),
            message: message.to_owned(),
        };
        let mut position = id3v2_len(bytes);
        let end = if bytes.len() >= 128 && bytes[bytes.len() - 128..].starts_with(b"TAG") {
            bytes.len() - 128
        } else {
            bytes.len()
        };
        let mut info: Option<Self> = None;
        let mut samples: u64 = 0;
        while position + 4 <= end {
            let header = &bytes[position..position + 4];
            let Some(frame) = Frame::parse(header) else {
                // Not a sync word here. Tolerate junk only before the first frame.
                if info.is_some() {
                    break;
                }
                position += 1;
                continue;
            };
            match &mut info {
                None => {
                    info = Some(Self {
                        seconds: 0.0,
                        bitrate_kbps: frame.bitrate_kbps,
                        constant_bitrate: true,
                        sample_rate: frame.sample_rate,
                        channels: frame.channels,
                        frames: 0,
                    });
                }
                Some(info) => {
                    if frame.bitrate_kbps != info.bitrate_kbps {
                        info.constant_bitrate = false;
                    }
                    if frame.sample_rate != info.sample_rate {
                        return Err(fail("sample rate changes between frames"));
                    }
                }
            }
            if let Some(info) = &mut info {
                info.frames += 1;
            }
            samples += u64::from(frame.samples);
            position += frame.len;
        }
        let mut info = info.ok_or_else(|| fail("no MPEG audio frames found"))?;
        #[expect(
            clippy::cast_precision_loss,
            reason = "sample counts stay far below 2^52"
        )]
        let played = samples as f64;
        info.seconds = played / f64::from(info.sample_rate);
        Ok(info)
    }

    /// Reads a file.
    pub fn read(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path).map_err(|error| Error::io(path, error))?;
        Self::parse(path, &bytes)
    }
}

/// Bytes an `ID3v2` tag occupies at the front, or 0.
fn id3v2_len(bytes: &[u8]) -> usize {
    if bytes.len() < 10 || !bytes.starts_with(b"ID3") {
        return 0;
    }
    let size = bytes[6..10]
        .iter()
        .fold(0usize, |acc, byte| (acc << 7) | usize::from(byte & 0x7F));
    let footer = if bytes[5] & 0x10 != 0 { 10 } else { 0 };
    10 + size + footer
}

struct Frame {
    len: usize,
    samples: u32,
    bitrate_kbps: u32,
    sample_rate: u32,
    channels: u8,
}

impl Frame {
    fn parse(header: &[u8]) -> Option<Self> {
        if header.len() < 4 || header[0] != 0xFF || header[1] & 0xE0 != 0xE0 {
            return None;
        }
        let version = (header[1] >> 3) & 0x03; // 0 = 2.5, 2 = 2, 3 = 1
        let layer = (header[1] >> 1) & 0x03; // 1 = III, 2 = II, 3 = I
        let bitrate_index = usize::from(header[2] >> 4);
        let rate_index = usize::from((header[2] >> 2) & 0x03);
        let padding = (header[2] >> 1) & 0x01;
        let channel_mode = header[3] >> 6;
        if version == 1
            || layer == 0
            || bitrate_index == 0
            || bitrate_index == 15
            || rate_index == 3
        {
            return None;
        }
        let mpeg1 = version == 3;
        let bitrate_kbps = match (mpeg1, layer) {
            (true, 1) => [
                0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320,
            ][bitrate_index],
            (true, 2) => [
                0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384,
            ][bitrate_index],
            (true, _) => [
                0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448,
            ][bitrate_index],
            (false, 1 | 2) => {
                [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160][bitrate_index]
            }
            (false, _) => [
                0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256,
            ][bitrate_index],
        };
        let sample_rate = match version {
            3 => [44_100, 48_000, 32_000][rate_index],
            2 => [22_050, 24_000, 16_000][rate_index],
            _ => [11_025, 12_000, 8_000][rate_index],
        };
        let samples: u32 = match (mpeg1, layer) {
            (_, 3) => 384,
            (true, _) | (false, 2) => 1152,
            (false, _) => 576,
        };
        let padding = u32::from(padding);
        let len = if layer == 3 {
            (12 * bitrate_kbps * 1000 / sample_rate + padding) * 4
        } else {
            samples / 8 * bitrate_kbps * 1000 / sample_rate + padding
        };
        Some(Self {
            len: len as usize,
            samples,
            bitrate_kbps,
            sample_rate,
            channels: if channel_mode == 3 { 1 } else { 2 },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One MPEG-1 Layer III frame header: 128 kbit/s, 44.1 kHz, stereo, no padding.
    const HEADER: [u8; 4] = [0xFF, 0xFB, 0x90, 0x00];

    fn frames(count: usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        for _ in 0..count {
            bytes.extend_from_slice(&HEADER);
            bytes.extend(std::iter::repeat_n(0u8, 417 - 4));
        }
        bytes
    }

    #[test]
    fn counts_frames_and_time() {
        let info = Mp3Info::parse(Path::new("x.mp3"), &frames(38)).unwrap();
        assert_eq!(info.frames, 38);
        assert_eq!(info.bitrate_kbps, 128);
        assert_eq!(info.sample_rate, 44_100);
        assert_eq!(info.channels, 2);
        assert!(info.constant_bitrate);
        assert!((info.seconds - 38.0 * 1152.0 / 44_100.0).abs() < 1e-9);
    }

    #[test]
    fn skips_id3_tags() {
        let mut bytes = b"ID3\x04\x00\x00\x00\x00\x00\x05hello".to_vec();
        bytes.extend(frames(2));
        bytes.extend_from_slice(b"TAG");
        bytes.extend(std::iter::repeat_n(0u8, 125));
        let info = Mp3Info::parse(Path::new("x.mp3"), &bytes).unwrap();
        assert_eq!(info.frames, 2);
    }

    #[test]
    fn refuses_non_audio() {
        assert!(Mp3Info::parse(Path::new("x.mp3"), b"not audio at all").is_err());
    }
}
