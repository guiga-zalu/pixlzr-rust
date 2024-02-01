use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Semver {
	pub major: u8,
	pub minor: u8,
	pub patch: u8,
}

impl Semver {
	pub const fn new(major: u8, minor: u8, patch: u8) -> Semver {
		Semver {
			major,
			minor,
			patch,
		}
	}
}

impl Ord for Semver {
	fn cmp(&self, other: &Self) -> Ordering {
		let m = (&self.major).cmp(&other.major);
		if m.is_eq() {
			let m = (&self.minor).cmp(&other.minor);
			if m.is_eq() {
				(&self.patch).cmp(&other.patch)
			} else {
				m
			}
		} else {
			m
		}
	}
}

impl PartialOrd for Semver {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl From<&[u8]> for Semver {
	fn from(value: &[u8]) -> Self {
		let len = value.len();
		let mut ver = Semver::default();
		if len > 0 {
			ver.major = value[0];
			if len > 1 {
				ver.minor = value[1];
				if len > 2 {
					ver.patch = value[2];
				}
			}
		}
		ver
	}
}

impl From<Box<[u8]>> for Semver {
	fn from(value: Box<[u8]>) -> Self {
		let len = value.len();
		let mut ver = Semver::default();
		if len > 0 {
			ver.major = value[0];
			if len > 1 {
				ver.minor = value[1];
				if len > 2 {
					ver.patch = value[2];
				}
			}
		}
		ver
	}
}

impl From<(u8, u8, u8)> for Semver {
	fn from((major, minor, patch): (u8, u8, u8)) -> Self {
		Semver {
			major,
			minor,
			patch,
		}
	}
}
