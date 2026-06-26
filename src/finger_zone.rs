use crate::protocols::Key;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FingerZone {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
    Thumb,
}

impl FingerZone {
    pub fn index(self) -> usize {
        match self {
            FingerZone::LeftPinky => 0,
            FingerZone::LeftRing => 1,
            FingerZone::LeftMiddle => 2,
            FingerZone::LeftIndex => 3,
            FingerZone::RightIndex => 4,
            FingerZone::RightMiddle => 5,
            FingerZone::RightRing => 6,
            FingerZone::RightPinky => 7,
            FingerZone::Thumb => 8,
        }
    }
}

/// Determine the finger zone for a key based on its physical position
/// within the keyboard layout. Wide keys (>=5 units) are classified as Thumb.
pub fn zone_by_position(key: &Key, all_keys: &[Key]) -> Option<FingerZone> {
    // Space bars and other wide thumb keys
    if key.w >= 5.0 {
        return Some(FingerZone::Thumb);
    }

    let min_x = all_keys
        .iter()
        .map(|k| k.x)
        .fold(f32::MAX, f32::min);
    let max_x = all_keys
        .iter()
        .map(|k| k.x + k.w)
        .fold(f32::MIN, f32::max);

    let total = max_x - min_x;
    if total <= 0.0 {
        return None;
    }

    let center = (key.x + key.w / 2.0) - min_x;
    let rel = center / total;

    // Boundaries derived from a standard ANSI 60% keyboard.
    if rel < 0.13 {
        Some(FingerZone::LeftPinky)
    } else if rel < 0.22 {
        Some(FingerZone::LeftRing)
    } else if rel < 0.30 {
        Some(FingerZone::LeftMiddle)
    } else if rel < 0.46 {
        Some(FingerZone::LeftIndex)
    } else if rel < 0.55 {
        Some(FingerZone::RightIndex)
    } else if rel < 0.64 {
        Some(FingerZone::RightMiddle)
    } else if rel < 0.73 {
        Some(FingerZone::RightRing)
    } else {
        Some(FingerZone::RightPinky)
    }
}
