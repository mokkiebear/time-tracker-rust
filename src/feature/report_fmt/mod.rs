use std::time::Duration;

#[derive(Debug, Default)]
pub struct HMSFormatter;

trait DurationFormatter {
    fn format(&self, duration: Duration) -> String;
}

impl DurationFormatter for HMSFormatter {
    fn format(&self, duration: Duration) -> String {
        let seconds = duration.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_seconds() {
        let duration = Duration::from_secs(5);

        let formatter = HMSFormatter::default();

        let text = formatter.format(duration);

        assert_eq!(text, "00:00:05");
    }
}
