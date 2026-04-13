use wasm_bindgen::prelude::*;
use js_sys::{Date, Intl, Array, Object, Reflect};
use std::collections::HashMap;

thread_local! {
    pub static TIMEZONES: HashMap<String, crate::models::TimezoneMetadata> = {
        let json = include_str!("../../src/data/timezones.json");
        let list: Vec<crate::models::TimezoneMetadata> = serde_json::from_str(json).unwrap_or_default();
        let mut map = HashMap::new();
        for tz in list {
            map.insert(tz.id.clone(), tz);
        }
        map
    };
}

pub fn get_timezone_metadata(iana_id: &str) -> Option<crate::models::TimezoneMetadata> {
    TIMEZONES.with(|map| map.get(iana_id).cloned())
}

pub fn format_time(timestamp: i64, time_zone: &str, is_12h: bool) -> String {
    let date = Date::new(&JsValue::from_f64(timestamp as f64));
    let options = Object::new();
    let _ = Reflect::set(&options, &JsValue::from_str("timeZone"), &JsValue::from_str(time_zone));
    let _ = Reflect::set(&options, &JsValue::from_str("hour"), &JsValue::from_str("numeric"));
    let _ = Reflect::set(&options, &JsValue::from_str("minute"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("second"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("hour12"), &JsValue::from_bool(is_12h));

    let formatter = Intl::DateTimeFormat::new(&Array::of1(&JsValue::from_str("en-US")), &options);
    formatter.format().call1(&JsValue::NULL, &date).unwrap().as_string().unwrap()
}

pub fn format_date(timestamp: i64, time_zone: &str) -> String {
    let date = Date::new(&JsValue::from_f64(timestamp as f64));
    let options = Object::new();
    let _ = Reflect::set(&options, &JsValue::from_str("timeZone"), &JsValue::from_str(time_zone));
    let _ = Reflect::set(&options, &JsValue::from_str("weekday"), &JsValue::from_str("short"));
    let _ = Reflect::set(&options, &JsValue::from_str("month"), &JsValue::from_str("short"));
    let _ = Reflect::set(&options, &JsValue::from_str("day"), &JsValue::from_str("numeric"));

    let formatter = Intl::DateTimeFormat::new(&Array::of1(&JsValue::from_str("en-US")), &options);
    formatter.format().call1(&JsValue::NULL, &date).unwrap().as_string().unwrap()
}

pub fn format_timezone_offset(time_zone: &str, timestamp: i64) -> String {
    let date = Date::new(&JsValue::from_f64(timestamp as f64));
    let options = Object::new();
    let _ = Reflect::set(&options, &JsValue::from_str("timeZone"), &JsValue::from_str(time_zone));
    let _ = Reflect::set(&options, &JsValue::from_str("timeZoneName"), &JsValue::from_str("shortOffset"));

    let formatter = Intl::DateTimeFormat::new(&Array::of1(&JsValue::from_str("en-US")), &options);
    let parts = formatter.format_to_parts(&date);
    
    // We can just format the whole string and it usually includes the date and offset like "1/1/2021, GMT+5"
    // Let's extract the timeZoneName part
    for i in 0..parts.length() {
        let part = parts.get(i);
        if let Ok(obj) = part.dyn_into::<Object>() {
            if let Ok(type_val) = Reflect::get(&obj, &JsValue::from_str("type")) {
                if type_val.as_string() == Some("timeZoneName".into()) {
                    if let Ok(val) = Reflect::get(&obj, &JsValue::from_str("value")) {
                        return val.as_string().unwrap_or_default();
                    }
                }
            }
        }
    }
    String::new()
}

pub fn get_timezone_offset_minutes(time_zone: &str, timestamp: i64) -> i32 {
    let date = Date::new(&JsValue::from_f64(timestamp as f64));
    let options = Object::new();
    let _ = Reflect::set(&options, &JsValue::from_str("timeZone"), &JsValue::from_str(time_zone));
    let _ = Reflect::set(&options, &JsValue::from_str("timeZoneName"), &JsValue::from_str("longOffset"));

    let formatter = Intl::DateTimeFormat::new(&Array::of1(&JsValue::from_str("en-US")), &options);
    let parts = formatter.format_to_parts(&date);
    
    let mut tz_str = String::new();
    for i in 0..parts.length() {
        let part = parts.get(i);
        if let Ok(obj) = part.dyn_into::<Object>() {
            if let Ok(type_val) = Reflect::get(&obj, &JsValue::from_str("type")) {
                if type_val.as_string() == Some("timeZoneName".into()) {
                    if let Ok(val) = Reflect::get(&obj, &JsValue::from_str("value")) {
                        tz_str = val.as_string().unwrap_or_default();
                    }
                }
            }
        }
    }

    if tz_str.is_empty() {
        return 0;
    }

    // Parse "GMT+05:30"
    if let Some(rest) = tz_str.strip_prefix("GMT") {
        if rest.is_empty() { return 0; }
        let sign = if rest.starts_with('+') { 1 } else { -1 };
        let parts: Vec<&str> = rest[1..].split(':').collect();
        if parts.len() == 2 {
            let h: i32 = parts[0].parse().unwrap_or(0);
            let m: i32 = parts[1].parse().unwrap_or(0);
            return sign * (h * 60 + m);
        }
    }
    0
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeZoneParts {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub weekday: u32,
}

impl TimeZoneParts {
    pub fn date_key(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

pub fn parse_alarm_time(target_time: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = target_time.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let hour = parts[0].parse().ok()?;
    let minute = parts[1].parse().ok()?;
    Some((hour, minute))
}

pub fn get_zoned_time_parts(timestamp: i64, time_zone: &str) -> Option<TimeZoneParts> {
    let date = Date::new(&JsValue::from_f64(timestamp as f64));
    let options = Object::new();
    let _ = Reflect::set(&options, &JsValue::from_str("timeZone"), &JsValue::from_str(time_zone));
    let _ = Reflect::set(&options, &JsValue::from_str("year"), &JsValue::from_str("numeric"));
    let _ = Reflect::set(&options, &JsValue::from_str("month"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("day"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("hour"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("minute"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("weekday"), &JsValue::from_str("short"));
    let _ = Reflect::set(&options, &JsValue::from_str("hourCycle"), &JsValue::from_str("h23"));

    let formatter = Intl::DateTimeFormat::new(&Array::of1(&JsValue::from_str("en-US")), &options);
    let parts = formatter.format_to_parts(&date);

    let mut year = None;
    let mut month = None;
    let mut day = None;
    let mut hour = None;
    let mut minute = None;
    let mut weekday = None;

    for i in 0..parts.length() {
        let part = parts.get(i);
        if let Ok(obj) = part.dyn_into::<Object>() {
            let part_type = Reflect::get(&obj, &JsValue::from_str("type")).ok()?.as_string()?;
            let value = Reflect::get(&obj, &JsValue::from_str("value")).ok()?.as_string()?;

            match part_type.as_str() {
                "year" => year = value.parse().ok(),
                "month" => month = value.parse().ok(),
                "day" => day = value.parse().ok(),
                "hour" => hour = value.parse().ok(),
                "minute" => minute = value.parse().ok(),
                "weekday" => {
                    weekday = match value.as_str() {
                        "Sun" => Some(0),
                        "Mon" => Some(1),
                        "Tue" => Some(2),
                        "Wed" => Some(3),
                        "Thu" => Some(4),
                        "Fri" => Some(5),
                        "Sat" => Some(6),
                        _ => None,
                    }
                }
                _ => {}
            }
        }
    }

    Some(TimeZoneParts {
        year: year?,
        month: month?,
        day: day?,
        hour: hour?,
        minute: minute?,
        weekday: weekday?,
    })
}

pub fn next_alarm_date_key(timestamp: i64, time_zone: &str, target_time: &str) -> Option<String> {
    let current = get_zoned_time_parts(timestamp, time_zone)?;
    let (target_hour, target_minute) = parse_alarm_time(target_time)?;

    if (current.hour, current.minute) <= (target_hour, target_minute) {
        return Some(current.date_key());
    }

    let tomorrow = get_zoned_time_parts(timestamp + 24 * 60 * 60 * 1000, time_zone)?;
    Some(tomorrow.date_key())
}
