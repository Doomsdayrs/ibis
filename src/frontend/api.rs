use crate::common::{ArticleView, LoginResponse, LoginUserData, RegisterUserData};
use crate::common::GetArticleData;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub async fn get_article(hostname: &str, title: String) -> ArticleView {
    let get_article = GetArticleData { title };
    get_query::<ArticleView, _>(hostname, "article", Some(get_article.clone())).await
}

pub async fn get_query<T, R>(hostname: &str, endpoint: &str, query: Option<R>) -> T
where
    T: for<'de> Deserialize<'de>,
    R: Serialize,
{
    let mut req = CLIENT.get(format!("http://{}/api/v1/{}", hostname, endpoint));
    if let Some(query) = query {
        req = req.query(&query);
    }
    handle_json_res::<T>(req).await
}

pub async fn handle_json_res<T>(req: RequestBuilder) -> T
where
    T: for<'de> Deserialize<'de>,
{
    let res = req.send().await.unwrap();
    let status = res.status();
    let text = res.text().await.unwrap();
    if status == reqwest::StatusCode::OK {
        serde_json::from_str(&text)
            .map_err(|e| anyhow!("Json error on {text}: {e}"))
            .unwrap()
    } else {
        Err(anyhow!("API error: {text}")).unwrap()
    }
}

pub async fn register(hostname: &str, username: &str, password: &str) -> LoginResponse {
    let register_form = RegisterUserData {
        username: username.to_string(),
        password: password.to_string(),
    };
    let req = CLIENT
        .post(format!("http://{}/api/v1/user/register", hostname))
        .form(&register_form);
    handle_json_res(req).await
}

pub async fn login(
    hostname: &str,
    username: &str,
    password: &str,
) -> LoginResponse {
    let login_form = LoginUserData {
        username: username.to_string(),
        password: password.to_string(),
    };
    let req = CLIENT
        .post(format!("http://{}/api/v1/user/login", hostname))
        .form(&login_form);
    handle_json_res(req).await
}
