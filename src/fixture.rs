use chrono::{Local, NaiveDate, TimeZone};
use serde_json::json;

use crate::debt::{DebtRequest, Partner};
use crate::debt_status::{DebtStatusRequest, DebtStatusVariable};
use crate::debtor::{DebtorRequest, Gender};

pub fn debtor_request_sample_data() -> DebtorRequest {
    DebtorRequest {
        debtor_id: "test-external-id".into(),
        name: "名前".into(),
        name_kana: "カナ".into(),
        birth_date: Some(NaiveDate::from_ymd_opt(1999, 1, 1).unwrap()),
        gender: Gender::Male,
        email: "sample@example.com".into(),
        address: "東京都xx 区xx町x-x-x".into(),
        kyc_done: true,
        postal_code: "3336666".into(),
        phone_number: "0312345678".into(),
        mobile_number: "09012345678".into(),
    }
}

pub fn debt_request_sample_data() -> DebtRequest {
    DebtRequest {
        debt_id: "1234-4321".into(),
        debtor_id: "5678".into(),
        dealt_at: Local.with_ymd_and_hms(2021, 12, 1, 12, 13, 0).unwrap(),
        debt_amount: 7400,
        debt_fee: Some(540),
        debt_delinquency_charge: Some(680),
        repayment_due_at: Local.with_ymd_and_hms(2022, 3, 1, 23, 59, 59).unwrap(),
        appendix: Some(
            r#"lease_id:xxxx lease_contract_id:xxxxx item_name:Windowsノートパソコン transaction_id:HGBVPKRN_1LCBU8F requester_name:ヤギ ナツキ total_amount:15240 elapsed_month:-2"#.into(),
        ),
        remind_segments: Some(vec!["y2021".into()]),
        partner: Some(Partner {
            id: "1234-5678".into(),
            name: "加盟店アメリケン".into(),
        }),
        debt_status: Some(DebtStatusRequest{
            debt_id: "1234-5678".into(),
            status: Some(DebtStatusVariable::Repaid),
            changed_at: Local.with_ymd_and_hms(2021, 11, 15, 12, 34, 0).unwrap(),
            expire_at: Local.with_ymd_and_hms(9999, 12, 31, 23, 59, 59).unwrap(),
            status_id: None,
        })
    }
}

pub fn debt_status_request_sample_data() -> DebtStatusRequest {
    DebtStatusRequest {
        debt_id: "1234-5678".into(),
        status: Some(DebtStatusVariable::Repaid),
        changed_at: Local.with_ymd_and_hms(2021, 11, 15, 12, 34, 0).unwrap(),
        expire_at: Local.with_ymd_and_hms(9999, 12, 31, 23, 59, 59).unwrap(),
        status_id: None,
    }
}

pub fn lecto_debtor_response() -> serde_json::Value {
    json!({
        "id": 111,
        "debtor_id": "DEBTOR_111",
        "basic_information": {
            "name": "name",
            "name_kana": "name kana",
            "birth_date": "1999-01-01",
            "gender": "male",
        },
        "email": {
            "email": "sample@example.com",
        },
        "address": {
            "address": "東京都xx区xx町x-x-x",
            "kyc_done": true,
            "postal_code": "3336666",
        },
        "phone_number": {
            "phone_number": "0312345678",
            "mobile_number": "09012345678",
        },
    })
}

pub fn lecto_debt_response() -> serde_json::Value {
    json!({
        "id": 1,
        "debt_id": "debt id",
        "debtor_id": "debtor id",
        "dealt_at": "2021-01-01T10:00:00+09:00",
        "debt_amount": 100,
        "debt_fee": 0,
        "debt_delinquency_charge": 10,
        "repayment_due_at": "2021-01-01T10:00:00+09:00",
        "appendix": "aaaaa",
        "remind_segments": [
            { "name": "seg-1" },
            { "name": "seg-2" },
        ],
        "debt_status": {
            "id": 10,
            "debt_id": "debt id",
            "changed_at": "2022-03-12T00:31:57+09:00",
            "expire_at": "9999-12-31T23:59:59+09:00",
            "status": "active",
            "status_id": "LECTO-01"
        }
    })
}

pub fn lecto_debt_status_response() -> serde_json::Value {
    json!({
        "id": 1,
        "debt_id": "debt id",
        "changed_at": "2021-01-01T10:00:00+09:00",
        "expire_at": "2021-01-01T10:00:00+09:00",
        "status": "repaid",
        "status_id": "LECTO-400"
    })
}
