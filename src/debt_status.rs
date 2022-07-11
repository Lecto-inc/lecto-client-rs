use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DebtStatusRequest {
    pub debt_id: String,
    pub status: String,
    pub changed_at: DateTime<Local>,
    pub expire_at: DateTime<Local>,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_serialize_reponse() -> anyhow::Result<()> {
        let req = DebtStatusRequest {
            debt_id: "1234-5678".into(),
            status: "repaid".into(),
            changed_at: Local.ymd(2021, 11, 15).and_hms(12, 34, 0),
            expire_at: Local.ymd(9999, 12, 31).and_hms(23, 59, 59),
        };

        let serialized = serde_json::to_string(&req)?;
        assert_eq!(
            serialized,
            r#"{"debt_id":"1234-5678","status":"repaid","changed_at":"2021-11-15T12:34:00+09:00","expire_at":"9999-12-31T23:59:59+09:00"}"#
        );

        Ok(())
    }
}
