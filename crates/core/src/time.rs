use std::time::{SystemTime, UNIX_EPOCH};

/// Unix milliseconds. Every `created_at` / `updated_at` column stores this.
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// `yyyy-mm` bucket used for the media directory layout, derived from unix millis.
///
/// Implemented locally (Howard Hinnant's civil-from-days) so the crate needs no
/// calendar dependency and the mapping is directly unit-testable.
pub fn year_month(ms: i64) -> String {
    let days = ms.div_euclid(86_400_000);
    let (y, m, _d) = civil_from_days(days);
    format!("{y:04}-{m:02}")
}

pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_is_january_1970() {
        assert_eq!(year_month(0), "1970-01");
        assert_eq!(civil_from_days(0), (1970, 1, 1));
    }

    #[test]
    fn known_timestamps_map_correctly() {
        // 2024-02-29T12:00:00Z — a leap day, the classic off-by-one trap.
        assert_eq!(
            civil_from_days(1_709_208_000_000i64 / 86_400_000),
            (2024, 2, 29)
        );
        assert_eq!(year_month(1_709_208_000_000), "2024-02");
        // 2000-03-01T00:00:00Z
        assert_eq!(year_month(951_868_800_000), "2000-03");
        // 1969-12-31T23:00:00Z — negative millis must floor, not truncate.
        assert_eq!(year_month(-3_600_000), "1969-12");
    }
}
