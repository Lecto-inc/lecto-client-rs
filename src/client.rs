use std::fmt::Debug;
use std::time::Duration;

use chrono::NaiveDate;
use reqwest::header::HeaderMap;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::util::join_url;
use crate::{Debt, DebtRequest, DebtStatus, DebtStatusRequest, Debtor, DebtorRequest};

use super::remind_group::remind::Remind;

/// TODO: debtorでもdebtsでも使えるようにrequest,responseをStringにしているがDebtor以外の型ができたタイミングでGenericsにしたほうがいい気がする
#[derive(thiserror::Error, Debug)]
pub enum LectoError {
    #[error("Status: {status} Res: {response:#?}")]
    UnprocessableEntity {
        status: StatusCode,
        request: String,
        response: String,
    },
    #[error("Status: {status} Res: {response:#?}")]
    BadRequest {
        status: StatusCode,
        request: String,
        response: String,
    },
    #[error("Status: {status} Res: {response:#?}")]
    InternalServerError {
        status: StatusCode,
        request: String,
        response: String,
    },
}

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
    max_retry: usize,
}

impl Client {
    pub fn new(api_key: String, base_url: String, max_retry: usize, timeout_secs: u64) -> Client {
        Client {
            api_key,
            base_url,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(timeout_secs))
                .build()
                .unwrap(),
            max_retry,
        }
    }

    pub async fn post_debtor(&self, req: DebtorRequest) -> anyhow::Result<Debtor> {
        let headers = self.common_headers()?;
        let url = join_url(&self.base_url, &["debtors"])?;

        let res = retry(
            || {
                let url = url.clone();
                let headers = headers.clone();
                self.client.post(url).json(&req).headers(headers).send()
            },
            self.max_retry,
        )
        .await?;

        Self::handle_response(Some(req), res).await
    }

    pub async fn post_debt(&self, req: DebtRequest) -> anyhow::Result<Debt> {
        let headers = self.common_headers()?;
        let url = join_url(&self.base_url, &["debts"])?;

        let res = retry(
            || {
                let url = url.clone();
                let headers = headers.clone();
                self.client.post(url).json(&req).headers(headers).send()
            },
            self.max_retry,
        )
        .await?;

        Self::handle_response(Some(req), res).await
    }

    pub async fn patch_debt_statuses(&self, req: DebtStatusRequest) -> anyhow::Result<DebtStatus> {
        let headers = self.common_headers()?;
        let url = join_url(&self.base_url, &["debt_statuses"])?;

        let res = retry(
            || {
                let url = url.clone();
                let headers = headers.clone();
                self.client.patch(url).json(&req).headers(headers).send()
            },
            self.max_retry,
        )
        .await?;

        Self::handle_response(Some(req), res).await
    }

    pub async fn get_reminds(
        &self,
        remind_group_id: u64,
        remind_at: NaiveDate,
    ) -> anyhow::Result<Vec<Remind>> {
        let headers = self.common_headers()?;
        let url = join_url(
            &self.base_url,
            &[
                "remind_groups",
                remind_group_id.to_string().as_str(),
                "reminds",
            ],
        )?;

        let res = retry(
            || {
                let url = url.clone();
                let headers = headers.clone();
                self.client
                    .get(url)
                    .headers(headers)
                    .query(&[
                        ("remind_at", remind_at.to_string().as_str()),
                        ("ignore_remind_group_status", "true"),
                    ])
                    .send()
            },
            self.max_retry,
        )
        .await?;

        Self::handle_response(None as Option<()>, res).await
    }

    fn common_headers(&self) -> anyhow::Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", self.api_key).parse()?);
        Ok(headers)
    }

    async fn handle_response<T: Debug, V: DeserializeOwned>(
        req: Option<T>,
        res: Response,
    ) -> anyhow::Result<V> {
        let status = res.status();
        if !status.is_success() {
            Err(match status {
                StatusCode::UNPROCESSABLE_ENTITY => LectoError::UnprocessableEntity {
                    status,
                    request: format!("{:#?}", req),
                    response: format!("{:#?}", res.text().await?),
                }
                .into(),
                StatusCode::BAD_REQUEST => LectoError::BadRequest {
                    status,
                    request: format!("{:#?}", req),
                    response: format!("{:#?}", res.text().await?),
                }
                .into(),
                StatusCode::INTERNAL_SERVER_ERROR => LectoError::InternalServerError {
                    status,
                    request: format!("{:#?}", req),
                    response: format!("{:#?}", res.text().await?),
                }
                .into(),
                _ => {
                    anyhow::anyhow!(
                        "Something else happened. Status: {:?} Req: {:#?} Res: {}",
                        status,
                        req,
                        res.text().await?
                    )
                }
            })
        } else {
            Ok(res.json().await?)
        }
    }
}

async fn retry<F, R>(send: F, max_retry: usize) -> anyhow::Result<reqwest::Response>
where
    R: core::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
    F: Fn() -> R,
{
    use tokio::time::sleep;

    let mut attempts = 1;
    loop {
        let res = send().await;
        if attempts == max_retry {
            return Ok(res?);
        }

        attempts += 1;
        match res {
            Ok(x) => {
                return Ok(x);
            }
            Err(e) => {
                log::error!(
                    "👻 Reqwest Error! will retry attempts: {}, Error: {:?}",
                    attempts,
                    e
                );
            }
        }
        sleep(Duration::from_millis(1000)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture::{
        self, lecto_debt_response, lecto_debt_status_response, lecto_debtor_response,
    };
    use assert_matches::assert_matches;
    use mockito::Matcher;
    use serde_json::json;

    fn mock_server() -> mockito::ServerGuard {
        mockito::Server::new()
    }

    #[tokio::test]
    async fn test_post_debtor() -> anyhow::Result<()> {
        let api_key = "apikey";
        let client = Client::new(api_key.into(), mockito::server_url(), 1, 10);
        let req = fixture::debtor_request_sample_data();
        let response_body = lecto_debtor_response();
        let mock = mockito::mock("POST", "/debtors")
            .with_status(200)
            .match_header("authorization", format!("Bearer {}", api_key).as_str())
            .match_body(serde_json::to_string(&req)?.as_str())
            .with_body(serde_json::to_string(&response_body)?.as_str())
            .create();

        let res = client.post_debtor(req).await?;
        assert_matches!(res, Debtor { .. });
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_post_debtor_validation_error() -> anyhow::Result<()> {
        let api_key = "apikey";
        let client = Client::new(api_key.into(), mockito::server_url(), 1, 10);
        let req = fixture::debtor_request_sample_data();
        let response_body = serde_json::to_string(&json!({
            "errors": ["UnprocessableEntity"],
        }))?;

        let mock = mockito::mock("POST", "/debtors")
            .with_status(422)
            .match_header("authorization", format!("Bearer {}", api_key).as_str())
            .match_body(serde_json::to_string(&req)?.as_str())
            .with_body(&response_body)
            .create();

        let res = client.post_debtor(req.clone()).await;
        assert_matches!(res, Err(e) => {
            if let Some(LectoError::UnprocessableEntity{status, ..}) = e.downcast_ref::<LectoError>() {
                assert_eq!(*status, StatusCode::UNPROCESSABLE_ENTITY);
            } else {
                panic!("is not LectoError");
            }
        });

        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_post_debtor_internal_server_error() -> anyhow::Result<()> {
        let api_key = "apikey";
        let client = Client::new(api_key.into(), mockito::server_url(), 1, 10);
        let req = fixture::debtor_request_sample_data();
        let response_body = serde_json::to_string(&json!({
            "errors": ["InternalServerError"],
        }))?;

        let mock = mockito::mock("POST", "/debtors")
            .with_status(500)
            .match_header("authorization", format!("Bearer {}", api_key).as_str())
            .match_body(serde_json::to_string(&req)?.as_str())
            .with_body(&response_body)
            .create();

        let res = client.post_debtor(req.clone()).await;
        assert_matches!(res, Err(e) => {
            if let Some(LectoError::InternalServerError{status, ..}) = e.downcast_ref::<LectoError>() {
                assert_eq!(*status, StatusCode::INTERNAL_SERVER_ERROR);
            } else {
                panic!("is not LectoError");
            }
        });

        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_post_debtor_reqwest_error() {
        let api_key = "apikey";
        let client = Client::new(api_key.into(), "https://awsedfghjk.aiueo".into(), 1, 20);

        let res = client
            .post_debtor(fixture::debtor_request_sample_data())
            .await;

        assert_matches!(res, Err(e) => {
            assert_matches!(e.downcast_ref::<reqwest::Error>(), Some(_));
        });
    }

    #[tokio::test]
    async fn test_post_debt() -> anyhow::Result<()> {
        let api_key = "apikey";
        let client = Client::new(api_key.into(), mockito::server_url(), 1, 20);
        let req = fixture::debt_request_sample_data();
        let mock = mockito::mock("POST", "/debts")
            .with_status(200)
            .match_header("authorization", format!("Bearer {}", api_key).as_str())
            .match_body(serde_json::to_string(&req)?.as_str())
            .with_body(serde_json::to_string(&lecto_debt_response())?)
            .create();

        let _ = client.post_debt(req).await?;

        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_post_debt_validation_error() -> anyhow::Result<()> {
        let api_key = "apikey";
        let client = Client::new(api_key.into(), mockito::server_url(), 1, 10);
        let req = fixture::debt_request_sample_data();
        let response_body = serde_json::to_string(&json!({
            "errors": ["UnprocessableEntity"],
        }))?;

        let mock = mockito::mock("POST", "/debts")
            .with_status(422)
            .match_header("authorization", format!("Bearer {}", api_key).as_str())
            .match_body(serde_json::to_string(&req)?.as_str())
            .with_body(&response_body)
            .create();

        let res = client.post_debt(req.clone()).await;
        assert_matches!(res, Err(e) => {
            if let Some(LectoError::UnprocessableEntity{status, ..}) = e.downcast_ref::<LectoError>() {
                assert_eq!(*status, StatusCode::UNPROCESSABLE_ENTITY);
            } else {
                panic!("is not LectoError");
            }
        });

        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_patch_debt_status() -> anyhow::Result<()> {
        let api_key = "apikey";
        let client = Client::new(api_key.into(), mockito::server_url(), 1, 10);
        let req = fixture::debt_status_request_sample_data();
        let mock = mockito::mock("PATCH", "/debt_statuses")
            .with_status(200)
            .match_header("authorization", format!("Bearer {}", api_key).as_str())
            .match_body(serde_json::to_string(&req)?.as_str())
            .with_body(serde_json::to_string(&lecto_debt_status_response())?)
            .create();

        let _ = client.patch_debt_statuses(req).await?;

        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_patch_debt_status_validation_error() -> anyhow::Result<()> {
        let api_key = "apikey";
        let client = Client::new(api_key.into(), mockito::server_url(), 1, 10);
        let req = fixture::debt_status_request_sample_data();
        let response_body = serde_json::to_string(&json!({
            "errors": ["UnprocessableEntity"],
        }))?;

        let mock = mockito::mock("PATCH", "/debt_statuses")
            .with_status(422)
            .match_header("authorization", format!("Bearer {}", api_key).as_str())
            .match_body(serde_json::to_string(&req)?.as_str())
            .with_body(&response_body)
            .create();

        let res = client.patch_debt_statuses(req.clone()).await;
        assert_matches!(res, Err(e) => {
            if let Some(LectoError::UnprocessableEntity{status, ..}) = e.downcast_ref::<LectoError>() {
                assert_eq!(*status, StatusCode::UNPROCESSABLE_ENTITY);
            } else {
                panic!("is not LectoError");
            }
        });

        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_get_reminds() -> anyhow::Result<()> {
        let api_key = "apikey";
        let client = Client::new(api_key.into(), mockito::server_url(), 1, 10);
        let remind_group_id = 1;
        let remind_at = NaiveDate::from_ymd_opt(2022, 2, 2).unwrap();
        let json = std::fs::read_to_string("test-data/lecto-remind-groups-reminds.json")
            .map_err(|e| dbg!(e))?;

        let mock = mockito::mock(
            "GET",
            format!("/remind_groups/{}/reminds", remind_group_id).as_str(),
        )
        .match_header("authorization", format!("Bearer {}", api_key).as_str())
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("remind_at".into(), "2022-02-02".into()),
            Matcher::UrlEncoded("ignore_remind_group_status".into(), "true".into()),
        ]))
        .with_status(200)
        .with_body(json)
        .create();

        let _ = client.get_reminds(remind_group_id, remind_at).await?;

        mock.assert();
        Ok(())
    }
}
