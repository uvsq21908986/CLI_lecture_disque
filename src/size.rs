use std::fmt;

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct Size(u64);

impl Size {
    pub fn new(bytes: u64) -> Self {
        Self(bytes)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KIB: f64 = 1024.0;
        const MIB: f64 = KIB * 1024.0;
        const GIB: f64 = MIB * 1024.0;
        const TIB: f64 = GIB * 1024.0;

        let bytes = self.0 as f64;

        if bytes < KIB {
            write!(f, "{:<5} B", bytes)
        } else if bytes < MIB {
            write!(f, "{:<3.1} KiB", bytes / KIB)
        } else if bytes < GIB {
            write!(f, "{:<3.1} MiB", bytes / MIB)
        } else if bytes < TIB {
            write!(f, "{:<3.1} GiB", bytes / GIB)
        } else {
            write!(f, "{:<3.1} TiB", bytes / TIB)
        }
    }
}

impl std::ops::Add for Size {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Size(self.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_size_creation() {
        let size = Size::new(1024);
        assert_eq!(size.0, 1024);
    }

    #[test]
    fn test_size_addition() {
        let size1 = Size::new(1024);
        let size2 = Size::new(2048);
        let result = size1 + size2;
        assert_eq!(result.0, 3072);
    }

    #[test]
    fn test_size_display() {
        let size = Size::new(1023);
        assert_eq!(format!("{}", size), "1023  B");

        let size = Size::new(2048);
        assert_eq!(format!("{}", size), "2.0 KiB");

        let size = Size::new(5 * 1024 * 1024);
        assert_eq!(format!("{}", size), "5.0 MiB");

        let size = Size::new(2 * 1024 * 1024 * 1024);
        assert_eq!(format!("{}", size), "2.0 GiB");

        let size = Size::new(3 * 1024 * 1024 * 1024 * 1024);
        assert_eq!(format!("{}", size), "3.0 TiB");
    }
}
