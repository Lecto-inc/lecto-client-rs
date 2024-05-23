use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
pub struct Debtor {
    pub id: u64,
    pub debtor_id: String,
    pub basic_information: DebtorBasicInformation,
    pub email: DebtorEmail,
    pub address: DebtorAddress,
    pub phone_number: DebtorPhoneNumber,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
pub struct DebtorBasicInformation {
    pub name: String,
    pub name_kana: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Gender,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
pub struct DebtorEmail {
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
pub struct DebtorAddress {
    pub address: String,
    pub kyc_done: bool,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
pub struct DebtorPhoneNumber {
    pub phone_number: Option<String>,
    pub mobile_number: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct DebtorRequest {
    pub debtor_id: String,
    pub name: String,
    pub name_kana: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<NaiveDate>,
    pub gender: Gender,
    pub email: String,
    pub address: String,
    pub kyc_done: bool,
    pub postal_code: String,
    pub phone_number: String,
    pub mobile_number: String,
}

// kyc_doneがintegerかboolかの違い
// 内部で使うための物で、外部には公開しない
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct DebtorRawRequest {
    pub debtor_id: String,
    pub name: String,
    pub name_kana: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<NaiveDate>,
    pub gender: Gender,
    pub email: String,
    pub address: String,
    pub kyc_done: KycDone,
    pub postal_code: String,
    pub phone_number: String,
    pub mobile_number: String,
}
impl From<DebtorRequest> for DebtorRawRequest {
    fn from(item: DebtorRequest) -> Self {
        Self {
            debtor_id: item.debtor_id,
            name: item.name,
            name_kana: item.name_kana,
            birth_date: item.birth_date,
            gender: item.gender,
            email: item.email,
            address: item.address,
            kyc_done: if item.kyc_done {
                KycDone::Done
            } else {
                KycDone::NotDone
            },
            postal_code: item.postal_code,
            phone_number: item.phone_number,
            mobile_number: item.mobile_number,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
pub struct DebtorAddressResponse {
    pub address: String,
    pub kyc_done: KycDone,
    pub postal_code: Option<String>,
}
impl From<DebtorAddressResponse> for DebtorAddress {
    fn from(item: DebtorAddressResponse) -> Self {
        Self {
            address: item.address,
            kyc_done: item.kyc_done == KycDone::Done,
            postal_code: item.postal_code,
        }
    }
}

// kyc_doneがintegerかboolかの違い
#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
pub struct DebtorResponse {
    pub id: u64,
    pub debtor_id: String,
    pub basic_information: DebtorBasicInformation,
    pub email: DebtorEmail,
    pub address: DebtorAddressResponse,
    pub phone_number: DebtorPhoneNumber,
}

impl From<DebtorResponse> for Debtor {
    fn from(item: DebtorResponse) -> Self {
        Self {
            id: item.id,
            debtor_id: item.debtor_id,
            basic_information: item.basic_information,
            email: item.email,
            address: DebtorAddress::from(item.address),
            phone_number: item.phone_number,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    None,
    Male,
    Female,
    Other,
}

impl Default for Gender {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr, Serialize_repr, Eq, Hash, Default)]
#[repr(u8)]
pub enum KycDone {
    Done = 1,

    #[default]
    NotDone = 0,
}

#[cfg(test)]
mod tests {
    use crate::fixture::lecto_debtor_response;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_serialize_request() -> anyhow::Result<()> {
        let req = DebtorRequest {
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
        };

        let serialized = serde_json::to_string(&req)?;
        assert_eq!(
            serialized,
            r#"{"debtor_id":"test-external-id","name":"名前","name_kana":"カナ","birth_date":"1999-01-01","gender":"male","email":"sample@example.com","address":"東京都xx 区xx町x-x-x","kyc_done":true,"postal_code":"3336666","phone_number":"0312345678","mobile_number":"09012345678"}"#
        );

        Ok(())
    }

    #[test]
    fn test_deserialize_response() -> anyhow::Result<()> {
        let res_json = serde_json::to_string(&lecto_debtor_response())?;
        let _debtor: DebtorResponse = serde_json::from_str(&res_json)?;
        Ok(())
    }
}
