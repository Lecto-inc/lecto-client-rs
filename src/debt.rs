use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{DebtStatus, DebtStatusRequest};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Debt {
    pub id: u64,
    pub debt_id: String,
    pub debtor_id: String,
    pub dealt_at: DateTime<Local>,
    pub debt_amount: i64,
    pub debt_fee: Option<i64>,
    pub debt_delinquency_charge: Option<i64>,
    pub repayment_due_at: DateTime<Local>,
    pub appendix: String,
    #[serde(default)]
    pub appendix_parsed: HashMap<String, String>,
    pub remind_segments: Vec<Segment>,
    #[serde(default)]
    pub partner: Option<Partner>,
    pub debt_status: DebtStatus,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Segment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DebtRequest {
    pub debt_id: String,
    pub debtor_id: String,
    pub dealt_at: DateTime<Local>,
    pub debt_amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debt_fee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debt_delinquency_charge: Option<i64>,
    pub repayment_due_at: DateTime<Local>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appendix: Option<String>,
    pub remind_segments: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<Partner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debt_status: Option<DebtStatusRequest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Partner {
    pub id: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::{fixture::lecto_debt_response, DebtStatusVariable};

    use super::*;
    use chrono::TimeZone;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_serialize_reponse() -> anyhow::Result<()> {
        let req = DebtRequest {
            debt_id: "1234-4321".into(),
            debtor_id: "5678".into(),
            dealt_at: Local.ymd(2021, 12, 1).and_hms(12, 13, 0),
            debt_amount: 7400,
            debt_fee: Some(540),
            debt_delinquency_charge: Some(680),
            repayment_due_at: Local.ymd(2022, 3, 1).and_hms(0, 0, 0),
            appendix: Some(
                r#"lease_id:xxxx lease_contract_id:xxxxx item_name:Windowsノートパソコン transaction_id:HGBVPKRN_1LCBU8F requester_name:ヤギ ナツキ total_amount:15240 elapsed_month:-2"#.into(),
            ),
            remind_segments: vec!["y2021".into()],
            partner: Some(Partner {
                id: "1234-5678".into(),
                name: "加盟店アメリケン".into(),
            }),
            debt_status: Some(DebtStatusRequest{
                debt_id: "1234-5678".into(),
                status: Some(DebtStatusVariable::Repaid),
                changed_at: Local.ymd(2021, 11, 15).and_hms(12, 34, 0),
                expire_at: Local.ymd(9999, 12, 31).and_hms(23, 59, 59),
                status_id: None,
            }),
        };

        let serialized = serde_json::to_string(&req)?;
        assert_eq!(
            serialized,
            r#"{"debt_id":"1234-4321","debtor_id":"5678","dealt_at":"2021-12-01T12:13:00+09:00","debt_amount":7400,"debt_fee":540,"debt_delinquency_charge":680,"repayment_due_at":"2022-03-01T00:00:00+09:00","appendix":"lease_id:xxxx lease_contract_id:xxxxx item_name:Windowsノートパソコン transaction_id:HGBVPKRN_1LCBU8F requester_name:ヤギ ナツキ total_amount:15240 elapsed_month:-2","remind_segments":["y2021"],"partner":{"id":"1234-5678","name":"加盟店アメリケン"},"debt_status":{"debt_id":"1234-5678","status":"repaid","changed_at":"2021-11-15T12:34:00+09:00","expire_at":"9999-12-31T23:59:59+09:00"}}"#
        );

        Ok(())
    }

    #[test]
    fn test_deseialize_response() -> anyhow::Result<()> {
        let res_json = serde_json::to_string(&lecto_debt_response())?;
        let _debt: Debt = serde_json::from_str(&res_json)?;
        Ok(())
    }
}
