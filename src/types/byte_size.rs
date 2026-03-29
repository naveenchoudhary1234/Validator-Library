use crate::error::ValidationError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteSize(pub u64);

impl ByteSize {
    pub fn bytes(self) -> u64 {
        self.0 // tuple first value extract karna
    }
}

impl FromStr for ByteSize {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ValidationError::InvalidByteSize {
                input: s.to_string(),
                reason: "input is empty".to_string(),
            });
        }

        let s_upper = s.to_uppercase();

        // Find where digits end and suffix begins
        let split_pos = s_upper.find(|c: char| c.is_alphabetic()).ok_or_else(|| {
            ValidationError::InvalidByteSize {
                input: s.to_string(),
                reason: "missing unit suffix (B, K, M, G, T)".to_string(),
            }
        })?;

        if split_pos == 0 {
            return Err(ValidationError::InvalidByteSize {
                input: s.to_string(),
                reason: "missing numeric value before suffix".to_string(),
            });
        }

        let number_part = &s_upper[..split_pos];
        let suffix_part = &s_upper[split_pos..];

        let number: u64 = number_part
            .parse()
            .map_err(|_| ValidationError::InvalidByteSize {
                input: s.to_string(),
                reason: format!("'{}' is not a valid positive integer", number_part),
            })?;

        if number == 0 {
            return Err(ValidationError::InvalidByteSize {
                input: s.to_string(),
                reason: "size must be non-zero".to_string(),
            });
        }

        let multiplier: u64 = match suffix_part {
            "B" => 1,
            "K" => 1_024,
            "M" => 1_048_576,
            "G" => 1_073_741_824,
            "T" => 1_099_511_627_776,
            _ => {
                return Err(ValidationError::InvalidByteSize {
                    input: s.to_string(),
                    reason: format!("unknown unit '{}', expected B, K, M, G, or T", suffix_part),
                })
            }
        };

        Ok(ByteSize(number * multiplier))
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const T: u64 = 1_099_511_627_776;
        const G: u64 = 1_073_741_824;
        const M: u64 = 1_048_576;
        const K: u64 = 1_024;

        let bytes = self.0;

        if bytes >= T && bytes.is_multiple_of(T) {
            write!(f, "{}T", bytes / T)
        } else if bytes >= G && bytes.is_multiple_of(G) {
            write!(f, "{}G", bytes / G)
        } else if bytes >= M && bytes.is_multiple_of(M) {
            write!(f, "{}M", bytes / M)
        } else if bytes >= K && bytes.is_multiple_of(K) {
            write!(f, "{}K", bytes / K)
        } else {
            write!(f, "{}B", bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_sizes() {
        assert_eq!("100G".parse::<ByteSize>().unwrap().0, 100 * 1_073_741_824);
        assert_eq!("512M".parse::<ByteSize>().unwrap().0, 512 * 1_048_576);
        assert_eq!("1T".parse::<ByteSize>().unwrap().0, 1_099_511_627_776);
        assert_eq!("1024K".parse::<ByteSize>().unwrap().0, 1024 * 1_024);
        assert_eq!("2048B".parse::<ByteSize>().unwrap().0, 2048);
    }

    #[test]
    fn test_parse_lowercase() {
        assert_eq!("100g".parse::<ByteSize>().unwrap().0, 100 * 1_073_741_824);
        assert_eq!("512m".parse::<ByteSize>().unwrap().0, 512 * 1_048_576);
    }

    #[test]
    fn test_reject_invalid() {
        assert!("".parse::<ByteSize>().is_err());
        assert!("abc".parse::<ByteSize>().is_err());
        assert!("-100G".parse::<ByteSize>().is_err());
        assert!("100X".parse::<ByteSize>().is_err());
        assert!("G100".parse::<ByteSize>().is_err());
        assert!("0G".parse::<ByteSize>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", "100G".parse::<ByteSize>().unwrap()), "100G");
        assert_eq!(format!("{}", "512M".parse::<ByteSize>().unwrap()), "512M");
        assert_eq!(format!("{}", "1T".parse::<ByteSize>().unwrap()), "1T");
    }
}
