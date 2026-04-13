use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct DayNightStatus {
    pub polar: bool,
    pub is_daylight: bool,
    pub sunrise: Option<DateTime<Utc>>,
    pub sunset: Option<DateTime<Utc>>,
}

pub fn get_day_night_status(lat: f64, lng: f64, date: DateTime<Utc>) -> Option<DayNightStatus> {
    if lat.is_nan() || lng.is_nan() || (lat == 0.0 && lng == 0.0) {
        return None;
    }

    let timestamp_ms = date.timestamp_millis();
    let pos = sun::pos(timestamp_ms, lat, lng);
    let is_daylight = pos.altitude > 0.0;

    // Find sunrise and sunset for the current UTC day
    let start_of_day = date.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let mut sunrise = None;
    let mut sunset = None;

    // Scan every 15 minutes to find zero crossings
    let mut prev_alt = sun::pos(start_of_day.timestamp_millis(), lat, lng).altitude;
    for minutes in (15..=(24 * 60)).step_by(15) {
        let t = start_of_day + Duration::minutes(minutes);
        let current_alt = sun::pos(t.timestamp_millis(), lat, lng).altitude;

        if prev_alt <= 0.0 && current_alt > 0.0 {
            // Sunrise binary search
            let exact = find_zero_crossing(t - Duration::minutes(15), t, lat, lng, true);
            sunrise = Some(exact);
        } else if prev_alt > 0.0 && current_alt <= 0.0 {
            // Sunset binary search
            let exact = find_zero_crossing(t - Duration::minutes(15), t, lat, lng, false);
            sunset = Some(exact);
        }

        prev_alt = current_alt;
    }

    let polar = sunrise.is_none() || sunset.is_none();

    Some(DayNightStatus {
        polar,
        is_daylight,
        sunrise,
        sunset,
    })
}

fn find_zero_crossing(mut start: DateTime<Utc>, mut end: DateTime<Utc>, lat: f64, lng: f64, rising: bool) -> DateTime<Utc> {
    for _ in 0..10 {
        let mid = start + (end - start) / 2;
        let alt = sun::pos(mid.timestamp_millis(), lat, lng).altitude;
        if rising {
            if alt > 0.0 {
                end = mid;
            } else {
                start = mid;
            }
        } else {
            if alt > 0.0 {
                start = mid;
            } else {
                end = mid;
            }
        }
    }
    start + (end - start) / 2
}
