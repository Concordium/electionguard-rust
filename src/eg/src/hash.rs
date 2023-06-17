// Copyright (C) Microsoft Corporation. All rights reserved.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]

use anyhow::anyhow;
use digest::{FixedOutput, Update};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use util::array_ascii::ArrayAscii;

type HmacSha256 = Hmac<sha2::Sha256>;

// "In ElectionGuard, all inputs that are used as the HMAC key, i.e. all inputs to the first
// argument of H have a fixed length of exactly 32 bytes."
// "The output of SHA-256 and therefore H is a 256-bit string, which can be interpreted as a
// byte array of 32 bytes."
pub const HVALUE_BYTE_LEN: usize = 32;
type HValueByteArray = [u8; HVALUE_BYTE_LEN];

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HValue(pub HValueByteArray);

impl HValue {
    const HVALUE_SERIALIZE_PREFIX: &[u8] = b"H(";
    const HVALUE_SERIALIZE_SUFFIX: &[u8] = b")";
    const HVALUE_SERIALIZE_LEN: usize = HValue::HVALUE_SERIALIZE_PREFIX.len()
        + HVALUE_BYTE_LEN * 2
        + HValue::HVALUE_SERIALIZE_SUFFIX.len();

    fn display_as_ascii(&self) -> ArrayAscii<{ HValue::HVALUE_SERIALIZE_LEN }> {
        enum State {
            Prefix(usize),
            Nibble { lower: bool, ix: usize },
            Suffix(usize),
            End,
        }
        let mut state = State::Prefix(0);
        ArrayAscii::from_fn(|_out_ix| match state {
            State::Prefix(ix) => {
                state = if ix + 1 < HValue::HVALUE_SERIALIZE_PREFIX.len() {
                    State::Prefix(ix + 1)
                } else {
                    State::Nibble {
                        lower: false,
                        ix: 0,
                    }
                };
                HValue::HVALUE_SERIALIZE_PREFIX[ix]
            }
            State::Nibble { lower, ix } => {
                let upper = !lower;
                let nibble = if upper {
                    state = State::Nibble { lower: upper, ix };
                    self.0[ix] >> 4
                } else {
                    state = if ix + 1 < HVALUE_BYTE_LEN {
                        State::Nibble {
                            lower: upper,
                            ix: ix + 1,
                        }
                    } else {
                        State::Suffix(0)
                    };
                    self.0[ix] & 0x0f
                };
                b"0123456789ABCDEF"[nibble as usize]
            }
            State::Suffix(ix) => {
                state = if ix + 1 < HValue::HVALUE_SERIALIZE_SUFFIX.len() {
                    State::Suffix(ix + 1)
                } else {
                    State::End
                };
                HValue::HVALUE_SERIALIZE_SUFFIX[ix]
            }
            State::End => {
                debug_assert!(false, "Should not be called after End state");
                b' '
            }
        })
    }
}

impl From<HValueByteArray> for HValue {
    #[inline]
    fn from(value: HValueByteArray) -> Self {
        HValue(value)
    }
}

impl From<&HValueByteArray> for HValue {
    #[inline]
    fn from(value: &HValueByteArray) -> Self {
        HValue(*value)
    }
}

impl AsRef<HValueByteArray> for HValue {
    #[inline]
    fn as_ref(&self) -> &HValueByteArray {
        &self.0
    }
}

impl std::fmt::Display for HValue {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(self.display_as_ascii().as_str())
    }
}

impl std::fmt::Debug for HValue {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use std::fmt::Write;

        let start_ix = HValue::HVALUE_SERIALIZE_PREFIX.len();
        let end_ix = HValue::HVALUE_SERIALIZE_LEN - HValue::HVALUE_SERIALIZE_SUFFIX.len();
        let ascii = &self.display_as_ascii();
        let hex_chars = &ascii.as_str()[start_ix..end_ix];

        f.write_str("HValue([")?;

        for (ix, hex_char) in hex_chars.chars().enumerate() {
            if ix % 2 == 0 {
                if ix > 0 {
                    f.write_str(", ")?;
                }
                f.write_str("0x")?;
            }
            f.write_char(hex_char)?;
        }

        f.write_str("])")
    }
}

impl Serialize for HValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.display_as_ascii().as_str().serialize(serializer)
    }
}

#[cfg(test)]
mod test_hvalue_std_fmt {
    use super::*;

    #[test]
    fn test_hvalue_std_fmt() {
        let h: HValue = std::array::from_fn(|ix| ix as u8).into();
        eprintln!("h Debug  : {h:?}");
        eprintln!("h Display: {h}");

        let expected = "H(000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F)";
        assert_eq!(h.to_string(), expected);
        assert_eq!(format!("{h}"), expected);

        let expected_debug = "HValue([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F])";
        assert_eq!(format!("{h:?}"), expected_debug);
    }
}

impl std::str::FromStr for HValue {
    type Err = anyhow::Error;

    /// Parses a string into an HValue.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix_start_ix = 0usize;
        let prefix_end_ix = HValue::HVALUE_SERIALIZE_PREFIX.len();

        let suffix_start_ix = HValue::HVALUE_SERIALIZE_LEN - HValue::HVALUE_SERIALIZE_SUFFIX.len();
        let suffix_end_ix = HValue::HVALUE_SERIALIZE_LEN;

        let hex_start_ix = prefix_end_ix;
        let hex_end_ix = suffix_start_ix;

        let bytes = s.as_bytes();

        let prefix_and_suffix_look_ok = bytes.len() == HValue::HVALUE_SERIALIZE_LEN
            && &bytes[prefix_start_ix..prefix_end_ix] == HValue::HVALUE_SERIALIZE_PREFIX
            && &bytes[suffix_start_ix..suffix_end_ix] == HValue::HVALUE_SERIALIZE_SUFFIX;

        let make_error = || anyhow!("Invalid HValue: {}", s);

        if !prefix_and_suffix_look_ok {
            return Err(make_error());
        }

        let hex_digits = &bytes[hex_start_ix..hex_end_ix];

        fn hex_digit_to_nibble(hex_digit: u8) -> Option<u8> {
            match hex_digit {
                b'0'..=b'9' => Some(hex_digit - b'0'),
                b'a'..=b'f' => Some(hex_digit - b'a' + 10),
                b'A'..=b'F' => Some(hex_digit - b'A' + 10),
                _ => None,
            }
        }

        let mut bad_digit = false;
        let mut byte_iterator = hex_digits.chunks_exact(2).map(|hex_digit_pair| {
            hex_digit_pair
                .iter()
                .map(|hex_digit| {
                    hex_digit_to_nibble(*hex_digit).unwrap_or_else(|| {
                        bad_digit = true;
                        0
                    })
                })
                .fold(0u8, |acc, hex_digit| (acc << 4) | hex_digit)
        });

        //? TODO Use std::array::array_try_from_fn when available https://github.com/rust-lang/rust/issues/89379
        let mut missing_digit = false;
        let hvba: HValueByteArray = std::array::from_fn(|_ix| {
            byte_iterator.next().unwrap_or_else(|| {
                missing_digit = true;
                0
            })
        });

        debug_assert!(byte_iterator.next().is_none());

        if bad_digit || missing_digit {
            return Err(make_error());
        }

        Ok(HValue(hvba))
    }
}

impl<'de> Deserialize<'de> for HValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;

        s.parse().map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod test_hvalue_serde_json {
    use super::*;

    #[test]
    fn test_hvalue_serde_json() {
        let h: HValue = std::array::from_fn(|ix| ix as u8).into();

        let json = serde_json::to_string(&h).unwrap();

        let expected = "\"H(000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F)\"";
        assert_eq!(json, expected);

        let h2: HValue = serde_json::from_str(&json).unwrap();
        assert_eq!(h2, h);
    }
}

// ElectionGuard "H" function.
pub fn eg_h(key: &HValue, data: &dyn AsRef<[u8]>) -> HValue {
    // `unwrap()` is justified here because `HmacSha256::new_from_slice()` seems
    // to only fail on slice of incorrect size.
    #[allow(clippy::unwrap_used)]
    let hmac_sha256 = HmacSha256::new_from_slice(key.as_ref()).unwrap();

    AsRef::<[u8; 32]>::as_ref(&hmac_sha256.chain(data).finalize_fixed()).into()
}

#[cfg(test)]
mod test_eg_h {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_evaluate_h() {
        let key: HValue = HValue::default();

        let data = [0u8; 0];

        let actual = eg_h(&key, &data);

        let expected =
            HValue::from_str("H(B613679A0814D9EC772F95D778C35FC5FF1697C493715653C6C712144292C5AD)")
                .unwrap();

        assert_eq!(actual, expected);
    }
}
