use serde::Deserialize;

use crate::{debt::Debt, debtor::Debtor};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Remind {
    pub label: String,
    pub debtor: Debtor,
    pub debts: Vec<Debt>,
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use crate::{
        debt::Segment,
        debtor::{DebtorAddress, DebtorBasicInformation, DebtorEmail, DebtorPhoneNumber, Gender},
        DebtStatus, DebtStatusVariable,
    };

    use super::*;

    #[test]
    fn test_deserialize() -> anyhow::Result<()> {
        let json = std::fs::read_to_string("test-data/lecto-remind-groups-reminds.json")
            .map_err(|e| dbg!(e))?;

        let data: Vec<Remind> = serde_json::from_str(&json).map_err(|e| dbg!(e))?;

        assert_matches!(&data[..], [first, ..] => {
            assert_eq!(first, &Remind {
                label: "test external id3---2022-03".into(),
                debtor: Debtor {
                    id: 38553,
                    debtor_id: "test external id3".into(),
                    basic_information: DebtorBasicInformation {
                        name: "test name".into(),
                        name_kana: Some("test name kana".into()),
                        birth_date: None,
                        gender: Gender::None,
                    },
                    email: DebtorEmail {
                        email: "sample@example.com".into()
                    },
                    address: DebtorAddress {
                        address: "東京都xx区xx町x-x-x".into(),
                        kyc_done: true,
                        postal_code: Some("3336666".into()),
                    },
                    phone_number: DebtorPhoneNumber {
                        phone_number: Some("0312345678".into()),
                        mobile_number: Some("09012345678".into()),
                    },
                },
                debts: vec![
                    Debt {
                        id: 27801,
                        debt_id: "test external id5".into(),
                        debtor_id: "test external id3".into(),
                        dealt_at: "2022-03-12T00:31:57+09:00".parse().unwrap(),
                        debt_amount: 10000,
                        debt_fee: Some(100),
                        debt_delinquency_charge: Some(500),
                        repayment_due_at: "2022-03-17T00:00:00+09:00".parse().unwrap(),
                        appendix: "test custom fields".into(),
                        appendix_parsed: [
                            ("item_name".into(), "iPhoneSE 12".into()),
                            ("total_amount".into(), "17000".into()),
                        ].into_iter().collect(),
                        custom_fields:  [
                            ("item_name".into(), "iPhoneSE 12".into()),
                            ("total_amount".into(), "17000".into()),
                        ].into_iter().collect(),
                        remind_segments: vec![Segment { name: "AAA".into() }],
                        partner: None,
                        debt_status: DebtStatus{
                            id: 10,
                            debt_id: "test external id5".into(),
                            changed_at: "2022-03-12T00:31:57+09:00".parse().unwrap(),
                            expire_at: "9999-12-31T23:59:59+09:00".parse().unwrap(),
                            status: DebtStatusVariable::DebtCancelled,
                            status_id: "LECTO-01".into()
                        },
                    },
                    Debt {
                        id: 27802,
                        debt_id: "test external id6".into(),
                        debtor_id: "test external id3".into(),
                        dealt_at: "2022-03-12T00:31:57+09:00".parse().unwrap(),
                        debt_amount: 10000,
                        debt_fee: Some(100),
                        debt_delinquency_charge: Some(500),
                        repayment_due_at: "2022-03-18T00:00:00+09:00".parse().unwrap(),
                        appendix: "test custom fields".into(),
                        appendix_parsed: [
                            ("item_name".into(), "iPhoneSE 12".into()),
                            ("total_amount".into(), "17000".into()),
                        ].into_iter().collect(),
                        custom_fields:  [
                            ("item_name".into(), "iPhoneSE 12".into()),
                            ("total_amount".into(), "17000".into()),
                        ].into_iter().collect(),
                        remind_segments: vec![Segment { name: "AAA".into() }],
                        partner: None,
                        debt_status: DebtStatus{
                            id: 11,
                            debt_id: "test external id6".into(),
                            changed_at: "2022-03-12T00:31:57+09:00".parse().unwrap(),
                            expire_at: "9999-12-31T23:59:59+09:00".parse().unwrap(),
                            status: DebtStatusVariable::Active,
                            status_id: "LECTO-02".into()
                        },
                    },
                ],
            })
        });
        Ok(())
    }
}
