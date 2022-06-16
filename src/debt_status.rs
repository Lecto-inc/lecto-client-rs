use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DebtStatusRequest {
    pub debt_id: String,
    pub status: String,
    pub changed_at: String,
    pub expire_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_reponse() -> anyhow::Result<()> {
        let req = DebtStatusRequest {
            debt_id: "1234-5678".into(),
            status: "repaid".into(),
            changed_at: "2021-11-15 12:34".into(),
            expire_at: "9999-12-31".into(),
        };

        let serialized = serde_json::to_string(&req)?;
        assert_eq!(
            serialized,
            r#"{"debt_id":"1234-5678","status":"repaid","changed_at":"2021-11-15 12:34","expire_at":"9999-12-31"}"#
        );

        Ok(())
    }
}
