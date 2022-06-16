use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{debt::Debt, debtor::Debtor};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Remind {
    pub label: String,
    pub debtor: Debtor,
    pub debts: Vec<Debt>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GroupedRemindRow {
    debtor_id: String,
    name: String,
    name_kana: Option<String>,
    postal_code: Option<String>,
    address: String,
    kyc_done: bool,
    phone_number: Option<String>,
    mobile_number: Option<String>,
    email: String,
    birth_date: Option<String>,
    gender: String,

    // 各debtの合計
    total_debt_amount: i64,
    total_debt_fee: Option<i64>,
    total_amount: i64,

    // 各debtのデータを結合したデータ
    debt_amount: String,
    debt_id: String,
    dealt_at: String,
    repayment_due_at: String,
    appendix: String,
    remind_segments: String,
}

pub fn grouping_reminds(reminds: Vec<Remind>) -> Vec<GroupedRemindRow> {
    reminds
        .into_iter()
        .into_group_map_by(|x| x.debtor.clone())
        .into_iter()
        .map(|(debtor, reminds)| {
            let debts = reminds.into_iter().flat_map(|x| x.debts).collect_vec();
            GroupedRemindRow {
                debtor_id: debtor.debtor_id,
                name: debtor.basic_information.name,
                name_kana: debtor.basic_information.name_kana,
                postal_code: debtor.address.postal_code,
                address: debtor.address.address,
                kyc_done: debtor.address.kyc_done,
                phone_number: debtor.phone_number.phone_number,
                mobile_number: debtor.phone_number.mobile_number,
                email: debtor.email.email,
                birth_date: debtor.basic_information.birth_date,
                gender: debtor.basic_information.gender,

                total_debt_amount: debts.iter().map(|x| x.debt_amount).sum(),
                total_debt_fee: Some(debts.iter().filter_map(|x| x.debt_fee).sum()),
                total_amount: debts
                    .iter()
                    .filter_map(|x| {
                        x.appendix_parsed
                            .get("total_amount")
                            .and_then(|x| x.parse::<i64>().ok())
                    })
                    .sum(),

                debt_amount: debts.iter().map(|x| x.debt_amount).join("\n"),
                debt_id: debts.iter().map(|x| &x.debt_id).join("\n"),
                dealt_at: debts.iter().map(|x| &x.dealt_at).join("\n"),
                repayment_due_at: debts.iter().map(|x| &x.repayment_due_at).join("\n"),
                appendix: debts.iter().map(|x| &x.appendix).join("\n---------\n"),
                remind_segments: debts
                    .iter()
                    .map(|x| x.remind_segments.iter().map(|s| &s.name).join(","))
                    .join("\n"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use crate::{
        debt::Segment,
        debtor::{DebtorAddress, DebtorBasicInformation, DebtorEmail, DebtorPhoneNumber},
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
                        gender: "none".into(),
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
                        dealt_at: "2022-03-12T00:31:57+09:00".into(),
                        debt_amount: 10000,
                        debt_fee: Some(100),
                        repayment_due_at: "2022-03-17T00:00:00+09:00".into(),
                        appendix: "test custom fields".into(),
                        appendix_parsed: [
                            ("item_name".into(), "iPhoneSE 12".into()),
                            ("total_amount".into(), "17000".into()),
                        ].into_iter().collect(),
                        remind_segments: vec![Segment { name: "AAA".into() }],
                    },
                    Debt {
                        id: 27802,
                        debt_id: "test external id6".into(),
                        debtor_id: "test external id3".into(),
                        dealt_at: "2022-03-12T00:31:57+09:00".into(),
                        debt_amount: 10000,
                        debt_fee: Some(100),
                        repayment_due_at: "2022-03-18T00:00:00+09:00".into(),
                        appendix: "test custom fields".into(),
                        appendix_parsed: [
                            ("item_name".into(), "iPhoneSE 12".into()),
                            ("total_amount".into(), "17000".into()),
                        ].into_iter().collect(),
                        remind_segments: vec![Segment { name: "AAA".into() }],
                    },
                ],
            })
        });
        Ok(())
    }

    #[test]
    fn test_remind_to_grouped_remind() {
        let remind = Remind {
            label: "test external id3---2022-03".into(),
            debtor: Debtor {
                id: 38553,
                debtor_id: "test external id3".into(),
                basic_information: DebtorBasicInformation {
                    name: "test name".into(),
                    name_kana: Some("test name kana".into()),
                    birth_date: None,
                    gender: "none".into(),
                },
                email: DebtorEmail {
                    email: "sample@example.com".into(),
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
                    dealt_at: "2022-03-12T00:31:57+09:00".into(),
                    debt_amount: 10000,
                    debt_fee: Some(100),
                    repayment_due_at: "2022-03-17T00:00:00+09:00".into(),
                    appendix: "test custom fields".into(),
                    appendix_parsed: [
                        ("item_name".into(), "iPhoneSE 12".into()),
                        ("total_amount".into(), "17000".into()),
                    ]
                    .into_iter()
                    .collect(),
                    remind_segments: vec![Segment { name: "AAA".into() }],
                },
                Debt {
                    id: 27802,
                    debt_id: "test external id6".into(),
                    debtor_id: "test external id3".into(),
                    dealt_at: "2022-03-12T00:31:57+09:00".into(),
                    debt_amount: 10000,
                    debt_fee: None,
                    repayment_due_at: "2022-03-18T00:00:00+09:00".into(),
                    appendix: "test custom fields".into(),
                    appendix_parsed: [
                        ("item_name".into(), "iPhoneSE 12".into()),
                        ("total_amount".into(), "17000".into()),
                    ]
                    .into_iter()
                    .collect(),
                    remind_segments: vec![Segment { name: "BBB".into() }],
                },
            ],
        };

        let grouped = grouping_reminds(vec![remind.clone(), remind.clone()]);

        assert_matches!(&grouped[..], [first] => {
            assert_eq!(first, &GroupedRemindRow {
                debtor_id: remind.debtor.debtor_id,
                name: remind.debtor.basic_information.name,
                name_kana: remind.debtor.basic_information.name_kana,
                postal_code: remind.debtor.address.postal_code,
                address: remind.debtor.address.address,
                kyc_done: remind.debtor.address.kyc_done,
                phone_number: remind.debtor.phone_number.phone_number,
                mobile_number: remind.debtor.phone_number.mobile_number,
                email: remind.debtor.email.email,
                birth_date: remind.debtor.basic_information.birth_date,
                gender: remind.debtor.basic_information.gender,

                total_debt_amount: 40000,
                total_debt_fee: Some(200),
                total_amount: 68000,

                debt_amount: "10000\n10000\n10000\n10000".into(),
                debt_id: "test external id5\ntest external id6\ntest external id5\ntest external id6".into(),
                dealt_at: "2022-03-12T00:31:57+09:00\n2022-03-12T00:31:57+09:00\n2022-03-12T00:31:57+09:00\n2022-03-12T00:31:57+09:00".into(),
                repayment_due_at: "2022-03-17T00:00:00+09:00\n2022-03-18T00:00:00+09:00\n2022-03-17T00:00:00+09:00\n2022-03-18T00:00:00+09:00".into(),
                appendix: "test custom fields\n---------\ntest custom fields\n---------\ntest custom fields\n---------\ntest custom fields".into(),
                remind_segments: "AAA\nBBB\nAAA\nBBB".into(),

            });
        });
    }
}
