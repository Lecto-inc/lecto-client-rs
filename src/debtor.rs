use serde::{Deserialize, Serialize};

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
    pub birth_date: Option<String>,
    pub gender: String,
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
    pub birth_date: String,
    pub gender: String,
    pub email: String,
    pub address: String,
    pub kyc_done: bool,
    pub postal_code: String,
    pub phone_number: String,
    pub mobile_number: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_serialize_reponse() -> anyhow::Result<()> {
        let req = DebtorRequest {
            debtor_id: "test-external-id".into(),
            name: "名前".into(),
            name_kana: "カナ".into(),
            birth_date: "1999-01-01".into(),
            gender: "male".into(),
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
}
