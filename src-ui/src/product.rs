use crate::models::{AlarmAttention, SystemInfo};
use delta_t_zaman_shared::AlarmAction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceCopy {
    pub title: String,
    pub body: String,
    pub action_label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BannerTone {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BannerCopy {
    pub tone: BannerTone,
    pub title: String,
    pub body: String,
}

pub fn banner_style(tone: &BannerTone) -> &'static str {
    match tone {
        BannerTone::Info => {
            "background: rgba(59, 130, 246, 0.08); color: #1d4ed8; border: 1px solid rgba(59, 130, 246, 0.18);"
        }
        BannerTone::Success => {
            "background: rgba(16, 185, 129, 0.08); color: #047857; border: 1px solid rgba(16, 185, 129, 0.18);"
        }
        BannerTone::Warning => {
            "background: rgba(245, 158, 11, 0.1); color: #b45309; border: 1px solid rgba(245, 158, 11, 0.2);"
        }
        BannerTone::Error => {
            "background: rgba(239, 68, 68, 0.08); color: #991b1b; border: 1px solid rgba(239, 68, 68, 0.18);"
        }
    }
}

fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {singular}")
    } else {
        format!("{count} {plural}")
    }
}

fn needs_verb(count: usize) -> &'static str {
    if count == 1 { "needs" } else { "need" }
}

pub fn app_boot_copy() -> SurfaceCopy {
    SurfaceCopy {
        title: "Preparing your workspace".into(),
        body: "Syncing clocks, alarms, and preferences...".into(),
        action_label: None,
    }
}

pub fn clocks_empty_copy() -> SurfaceCopy {
    SurfaceCopy {
        title: "No clocks yet".into(),
        body: "Add your home city, your team, or the places you check every day.".into(),
        action_label: Some("+ Add Timezone".into()),
    }
}

pub fn alarms_empty_copy() -> SurfaceCopy {
    SurfaceCopy {
        title: "No alarms scheduled".into(),
        body: "Create a one-time wake-up or a recurring reminder with its own timezone.".into(),
        action_label: Some("+ Add Alarm".into()),
    }
}

pub fn timezone_search_copy(query: &str, result_count: usize) -> SurfaceCopy {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return SurfaceCopy {
            title: "Search by city, country, or UTC offset".into(),
            body: "Try Tokyo, Brazil, or UTC+9 to quickly find the timezone you want.".into(),
            action_label: None,
        };
    }

    if result_count == 0 {
        return SurfaceCopy {
            title: format!("No matches for \"{trimmed}\""),
            body: "Try a nearby city, a country name, or the canonical timezone like Europe/Paris."
                .into(),
            action_label: None,
        };
    }

    let noun = if result_count == 1 { "match" } else { "matches" };
    SurfaceCopy {
        title: format!("{result_count} {noun} ready"),
        body: "Use arrow keys to move through results and press Enter to add the selected timezone."
            .into(),
        action_label: None,
    }
}

pub fn confirm_delete_clock_message(label: &str) -> String {
    format!("Remove \"{label}\" from your clocks? This only removes it from the dashboard.")
}

pub fn confirm_delete_alarm_message(title: &str) -> String {
    format!("Delete the alarm \"{title}\"? You can always create it again later.")
}

pub fn normalized_clock_label(input: &str, fallback: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn system_info_rows(info: &SystemInfo) -> Vec<(String, String)> {
    vec![
        ("App Version".into(), info.app_version.clone()),
        ("Platform".into(), format!("{} ({})", info.platform, info.arch)),
        ("Locale".into(), info.locale.clone()),
        ("Timezone".into(), info.timezone.clone()),
        (
            "Tray Support".into(),
            if info.supports_tray {
                "Available".into()
            } else {
                "Unavailable".into()
            },
        ),
        (
            "Device Class".into(),
            if info.is_mobile {
                "Mobile".into()
            } else {
                "Desktop".into()
            },
        ),
        ("OS Family".into(), info.os_version.clone()),
    ]
}

pub fn session_summary_copy(
    zone_count: usize,
    alarm_count: usize,
    attention: &AlarmAttention,
) -> SurfaceCopy {
    let title = if attention.has_attention() {
        format!(
            "{} {} attention",
            pluralize(attention.attention_count(), "alarm", "alarms"),
            needs_verb(attention.attention_count())
        )
    } else if zone_count == 0 && alarm_count == 0 {
        "Your workspace is ready to set up".into()
    } else {
        format!(
            "{} and {} in sync",
            pluralize(zone_count, "clock", "clocks"),
            pluralize(alarm_count, "alarm", "alarms")
        )
    };

    let body = if attention.has_attention() {
        format!(
            "Open the Alarms tab to handle {} ringing or snoozed reminders while {} stay visible in the dashboard.",
            pluralize(attention.attention_count(), "alarm", "alarms"),
            pluralize(zone_count, "clock", "clocks")
        )
    } else if zone_count == 0 && alarm_count == 0 {
        "Start with a timezone, then add alarms that match the places and schedules you care about."
            .into()
    } else {
        "Everything has loaded cleanly and there are no alarms waiting for action right now.".into()
    };

    SurfaceCopy {
        title,
        body,
        action_label: None,
    }
}

pub fn alarm_action_feedback_copy(
    action: AlarmAction,
    alarm_title: &str,
    attention: &AlarmAttention,
) -> BannerCopy {
    let (tone, title) = match action {
        AlarmAction::Snooze => (BannerTone::Info, format!("\"{alarm_title}\" snoozed")),
        AlarmAction::Dismiss => (BannerTone::Success, format!("\"{alarm_title}\" dismissed")),
    };

    let body = if attention.has_attention() {
        format!(
            "{} still {} attention.",
            pluralize(attention.attention_count(), "alarm", "alarms"),
            needs_verb(attention.attention_count())
        )
    } else {
        "No alarms need attention now.".into()
    };

    BannerCopy { tone, title, body }
}

pub fn alarm_delete_feedback_copy(alarm_title: &str, remaining_count: usize) -> BannerCopy {
    BannerCopy {
        tone: BannerTone::Success,
        title: format!("\"{alarm_title}\" deleted"),
        body: if remaining_count == 0 {
            "Your alarm list is clear for now.".into()
        } else {
            format!(
                "{} remain on the schedule.",
                pluralize(remaining_count, "alarm", "alarms")
            )
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_copy_guides_empty_and_failed_queries() {
        let idle = timezone_search_copy("", 0);
        assert!(idle.title.contains("Search by city"));

        let empty = timezone_search_copy("mars", 0);
        assert!(empty.title.contains("No matches"));
        assert!(empty.body.contains("Europe/Paris"));
    }

    #[test]
    fn search_copy_describes_ready_results() {
        let copy = timezone_search_copy("paris", 3);
        assert_eq!(copy.title, "3 matches ready");
        assert!(copy.body.contains("arrow keys"));
    }

    #[test]
    fn delete_messages_and_label_normalization_are_user_safe() {
        assert!(confirm_delete_clock_message("Paris").contains("Paris"));
        assert!(confirm_delete_alarm_message("Wake Up").contains("Wake Up"));
        assert_eq!(normalized_clock_label("   ", "Fallback"), "Fallback");
        assert_eq!(normalized_clock_label("  Team HQ  ", "Fallback"), "Team HQ");
    }

    #[test]
    fn system_rows_present_human_facing_values() {
        let rows = system_info_rows(&SystemInfo {
            platform: "linux".into(),
            arch: "x86_64".into(),
            locale: "en-US".into(),
            timezone: "UTC".into(),
            app_version: "1.2.3".into(),
            is_mobile: false,
            supports_tray: true,
            os_version: "linux".into(),
        });

        assert!(rows.iter().any(|(label, value)| label == "Tray Support" && value == "Available"));
        assert!(rows.iter().any(|(label, value)| label == "Platform" && value.contains("x86_64")));
    }

    #[test]
    fn session_summary_mentions_attention_when_needed() {
        let attention = AlarmAttention {
            triggered_alarm_ids: vec!["a1".into()],
            snoozed_alarm_ids: vec!["a2".into()],
        };

        let copy = session_summary_copy(3, 4, &attention);
        assert!(copy.title.contains("2 alarms need attention"));
        assert!(copy.body.contains("Alarms tab"));
    }

    #[test]
    fn session_summary_has_ready_state_for_empty_workspace() {
        let copy = session_summary_copy(0, 0, &AlarmAttention::default());
        assert!(copy.title.contains("ready to set up"));
        assert!(copy.body.contains("Start with a timezone"));
    }

    #[test]
    fn alarm_action_feedback_tracks_remaining_attention() {
        let feedback = alarm_action_feedback_copy(
            AlarmAction::Snooze,
            "Morning",
            &AlarmAttention {
                triggered_alarm_ids: vec!["a1".into()],
                snoozed_alarm_ids: vec!["a2".into()],
            },
        );

        assert_eq!(feedback.tone, BannerTone::Info);
        assert!(feedback.title.contains("Morning"));
        assert!(feedback.body.contains("2 alarms still need attention"));
    }

    #[test]
    fn banner_styles_exist_for_every_tone() {
        assert!(banner_style(&BannerTone::Info).contains("border"));
        assert!(banner_style(&BannerTone::Success).contains("border"));
        assert!(banner_style(&BannerTone::Warning).contains("border"));
        assert!(banner_style(&BannerTone::Error).contains("border"));
    }
}
