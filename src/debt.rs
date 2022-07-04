use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Debt {
    pub id: u64,
    pub debt_id: String,
    pub debtor_id: String,
    pub dealt_at: String,
    pub debt_amount: i64,
    pub debt_fee: Option<i64>,
    pub repayment_due_at: String,
    pub appendix: String,
    pub appendix_parsed: HashMap<String, String>,
    pub remind_segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Segment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct DebtRequest {
    pub debt_id: String,
    pub debtor_id: String,
    pub dealt_at: String,
    pub debt_amount: u64,
    pub debt_fee: Option<u64>,
    pub repayment_due_at: String,
    pub appendix: Option<String>,
    pub remind_segments: Vec<String>,
    pub partner: Option<Partner>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct Partner {
    pub id: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_serialize_reponse() -> anyhow::Result<()> {
        let req = DebtRequest {
            debt_id: "1234-4321".into(),
            debtor_id: "5678".into(),
            dealt_at: "2021-12-01 12:13".into(),
            debt_amount: 7400,
            debt_fee: Some(540),
            repayment_due_at: "2022-03-01".into(),
            appendix: Some(
                r#"lease_id:xxxx lease_contract_id:xxxxx item_name:Windowsノートパソコン transaction_id:HGBVPKRN_1LCBU8F requester_name:ヤギ ナツキ total_amount:15240 elapsed_month:-2"#.into(),
            ),
            remind_segments: vec!["y2021".into()],
            partner: Some(Partner {
                id: "1234-5678".into(),
                name: "加盟店アメリケン".into(),
            }),
        };

        let serialized = serde_json::to_string(&req)?;
        assert_eq!(
            serialized,
            r#"{"debt_id":"1234-4321","debtor_id":"5678","dealt_at":"2021-12-01 12:13","debt_amount":7400,"debt_fee":540,"repayment_due_at":"2022-03-01","appendix":"lease_id:xxxx lease_contract_id:xxxxx item_name:Windowsノートパソコン transaction_id:HGBVPKRN_1LCBU8F requester_name:ヤギ ナツキ total_amount:15240 elapsed_month:-2","remind_segments":["y2021"],"partner":{"id":"1234-5678","name":"加盟店アメリケン"}}"#
        );

        Ok(())
    }
}
