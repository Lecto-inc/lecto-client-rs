use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DebtStatusVariable {
    Active,
    AutoActivated,
    Repaid,
    DebtCancelled,
    BadDebtFixed,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DebtStatus {
    pub id: i64,
    pub debt_id: String,
    pub changed_at: DateTime<Local>,
    pub expire_at: DateTime<Local>,
    pub status: DebtStatusVariable,
    pub status_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DebtStatusRequest {
    pub debt_id: String,
    pub status_id: Option<String>,
    pub status: Option<DebtStatusVariable>,
    pub changed_at: DateTime<Local>,
    pub expire_at: DateTime<Local>,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use pretty_assertions::assert_eq;

    use crate::fixture::lecto_debt_status_response;

    use super::*;

    #[test]
    fn test_serialize_reponse() -> anyhow::Result<()> {
        let req = DebtStatusRequest {
            debt_id: "1234-5678".into(),
            status: Some(DebtStatusVariable::Repaid),
            changed_at: Local.ymd(2021, 11, 15).and_hms(12, 34, 0),
            expire_at: Local.ymd(9999, 12, 31).and_hms(23, 59, 59),
            status_id: Some("LECTO-001".into()),
        };

        let serialized = serde_json::to_string(&req)?;
        assert_eq!(
            serialized,
            r#"{"debt_id":"1234-5678","status_id":"LECTO-001","status":"repaid","changed_at":"2021-11-15T12:34:00+09:00","expire_at":"9999-12-31T23:59:59+09:00"}"#
        );

        Ok(())
    }

    #[test]
    fn test_deserialize_response() -> anyhow::Result<()> {
        let res_json = serde_json::to_string(&lecto_debt_status_response())?;
        let _debt_status: DebtStatus = serde_json::from_str(&res_json)?;
        Ok(())
    }
}
