use crate::models::{AlarmAttention, ExportResult, ImportResult};
use crate::product::{BannerCopy, BannerTone, SurfaceCopy};
use delta_t_zaman_shared::AlarmAction;

pub fn session_summary(
    zone_count: usize,
    alarm_count: usize,
    attention: &AlarmAttention,
) -> SurfaceCopy {
    crate::product::session_summary_copy(zone_count, alarm_count, attention)
}

pub fn attention_banner(attention: &AlarmAttention) -> Option<BannerCopy> {
    attention.has_attention().then(|| BannerCopy {
        tone: BannerTone::Warning,
        title: format!(
            "{} need attention",
            if attention.attention_count() == 1 {
                "1 alarm".to_string()
            } else {
                format!("{} alarms", attention.attention_count())
            }
        ),
        body: format!(
            "{} ringing, {} snoozed. Open alarms here to snooze or dismiss them.",
            attention.triggered_count(),
            attention.snoozed_count()
        ),
    })
}

pub fn alarm_action_notice(
    action: AlarmAction,
    alarm_title: &str,
    attention: &AlarmAttention,
) -> BannerCopy {
    crate::product::alarm_action_feedback_copy(action, alarm_title, attention)
}

pub fn alarm_delete_notice(alarm_title: &str, remaining_count: usize) -> BannerCopy {
    crate::product::alarm_delete_feedback_copy(alarm_title, remaining_count)
}

pub fn export_notice(result: &ExportResult) -> BannerCopy {
    BannerCopy {
        tone: BannerTone::Success,
        title: "Export complete".into(),
        body: format!(
            "Saved {} bytes to {}.",
            result.bytes_written, result.file_path
        ),
    }
}

pub fn import_notice(result: &ImportResult) -> BannerCopy {
    let mut body = format!(
        "Imported {} clocks and {} alarms. Preferences updated: {}.",
        result.zones_imported,
        result.alarms_imported,
        if result.preferences_updated { "yes" } else { "no" }
    );

    if !result.warnings.is_empty() {
        body.push(' ');
        body.push_str(&result.warnings.join(" "));
    }

    BannerCopy {
        tone: if result.conflicts_detected > 0 {
            BannerTone::Warning
        } else {
            BannerTone::Success
        },
        title: if result.conflicts_detected > 0 {
            "Import completed with warnings".into()
        } else {
            "Import complete".into()
        },
        body,
    }
}

pub fn cancelled_notice(action: &str, stage: &'static str) -> BannerCopy {
    BannerCopy {
        tone: BannerTone::Info,
        title: format!("{action} cancelled"),
        body: format!("{action} cancelled before {stage}."),
    }
}

pub fn error_notice(action: &str, details: &str) -> BannerCopy {
    BannerCopy {
        tone: BannerTone::Error,
        title: format!("{action} failed"),
        body: details.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_workflow_surfaces_recovered_attention() {
        let attention = AlarmAttention {
            triggered_alarm_ids: vec!["a1".into()],
            snoozed_alarm_ids: vec!["a2".into()],
        };

        let summary = session_summary(3, 2, &attention);
        let banner = attention_banner(&attention).expect("attention banner");

        assert!(summary.title.contains("2 alarms need attention"));
        assert_eq!(banner.tone, BannerTone::Warning);
        assert!(banner.body.contains("1 ringing, 1 snoozed"));
    }

    #[test]
    fn import_workflow_reports_merge_warning() {
        let banner = import_notice(&ImportResult {
            success: true,
            zones_imported: 4,
            alarms_imported: 2,
            preferences_updated: true,
            conflicts_detected: 1,
            warnings: vec!["Skipped 1 conflicting alarm entry during merge.".into()],
        });

        assert_eq!(banner.tone, BannerTone::Warning);
        assert!(banner.title.contains("warnings"));
        assert!(banner.body.contains("Skipped 1 conflicting alarm entry"));
    }

    #[test]
    fn alarm_action_workflow_clears_attention_message() {
        let banner = alarm_action_notice(
            AlarmAction::Dismiss,
            "Morning",
            &AlarmAttention::default(),
        );

        assert_eq!(banner.tone, BannerTone::Success);
        assert!(banner.title.contains("Morning"));
        assert!(banner.body.contains("No alarms need attention now"));
    }

    #[test]
    fn cancel_and_error_workflows_are_human_readable() {
        let cancelled = cancelled_notice("Import", "a mode was selected");
        let error = error_notice("Export", "Permission denied");

        assert_eq!(cancelled.tone, BannerTone::Info);
        assert!(cancelled.body.contains("before a mode was selected"));
        assert_eq!(error.tone, BannerTone::Error);
        assert!(error.body.contains("Permission denied"));
    }
}
