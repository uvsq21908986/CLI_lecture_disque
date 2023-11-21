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
            write!(f, "{:<5.1} KiB", bytes / KIB)
        } else if bytes < GIB {
            write!(f, "{:<5.1} MiB", bytes / MIB)
        } else if bytes < TIB {
            write!(f, "{:<5.1} GiB", bytes / GIB)
        } else {
            write!(f, "{:<5.1} TiB", bytes / TIB)
        }
    }
}

impl std::ops::Add for Size {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Size(self.0 + other.0)
    }
}
