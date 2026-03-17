#![allow(dead_code)]

use serde_json::json;
use tongbal_api::{AccessToken, AppClient, ClientId, ClientSecret};
use url::Url;
use wiremock::{
    Mock, MockBuilder, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path, query_param},
};

#[derive(Debug)]
pub struct AppHttpMock {
    app_client: AppClient,
    server: MockServer,
}

impl AppHttpMock {
    pub async fn new() -> Self {
        let server = MockServer::start().await;
        let mut url_parsed = Url::parse(&server.uri()).unwrap();
        url_parsed.set_path("/open/v1");

        let app_client = AppClient::with_client_builder(
            ClientId::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"),
            ClientSecret::from("VeIMuc9XGle7PSxIVYNwPpI2OEk_9gXoW_XXXXXXXXX"),
            reqwest::ClientBuilder::new(),
        )
        .with_url(url_parsed);

        Self { app_client, server }
    }

    pub fn client(&self) -> &AppClient {
        &self.app_client
    }
}

impl AppHttpMock {
    pub async fn search_category(&self) {
        app_client_mock("get", "/categories/search")
       .and(query_param("query", "all"))
        .and(query_param("size", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
  "code": 200,
  "message": null,
  "content": {
    "data": [
      {
        "categoryType": "GAME",
        "categoryId": "Total_Football",
        "categoryValue": "Total Football - 25/26 시즌",
        "posterImageUrl": null
      },
      {
        "categoryType": "GAME",
        "categoryId": "BATTLE_X_Birth_of_the_Alliance",
        "categoryValue": "BATTLE X : Birth of the Alliance",
        "posterImageUrl": "https://nng-phinf.pstatic.net/MjAyNTAxMDZfMzEg/MDAxNzM2MTQ0ODM0Njc0.pvZgOYjAOuG7S7-jkssTs0SlFUYukeW8yZPj9dCVsHYg.qNVmvNi4P_BKxPm52B-bGZ48E896bF0jHUTWQKlldzIg.JPEG/%EB%B0%B0%ED%8B%80.jpg"
      },
      {
        "categoryType": "GAME",
        "categoryId": "Touhou_Juuouen",
        "categoryValue": "동방수왕원 ~ Unfinished Dream of All Living Ghost.",
        "posterImageUrl": "https://nng-phinf.pstatic.net/MjAyNTA5MDhfMTI1/MDAxNzU3MzE0MDQwODQz.FcV7GTVWG5BNaA7EhFctkTUG3kMrY2dFjoz6ZchR1KMg.XG8g8IqWzBzWuo4P101rHrEaLgi5n4l62ULxzMesdCcg.PNG/%EB%8F%99%EB%B0%A9%EC%88%98%EC%99%95%EC%9B%90_%ED%8F%AC%EC%8A%A4%ED%84%B0.png"
      },
      {
        "categoryType": "GAME",
        "categoryId": "All_Aboard_The_Train_Defense_Express",
        "categoryValue": "All Aboard! 기차 방어 특급",
        "posterImageUrl": "https://nng-phinf.pstatic.net/MjAyNTAzMDdfMTM1/MDAxNzQxMzM5NTI0ODQz.7KsT7UKm96AXkFQkELMafKfR-mm0s_vD6MURoWZDwrIg.AGxxJD0-xYPdV7W5dgmSNQczgiauoaEcbB-f8M1_O5Ig.PNG/All_Aboard_%EA%B8%B0%EC%B0%A8_%EB%B0%A9%EC%96%B4_%ED%8A%B9%EA%B8%89.png"
      },
      {
        "categoryType": "GAME",
        "categoryId": "Power_Pros_2020",
        "categoryValue": "eBASEBALL 파워풀 프로야구 2020",
        "posterImageUrl": null
      }
    ]
  }
})))
        .mount(&self.server)
        .await;
    }
}

impl AppHttpMock {
    pub async fn get_live_list(&self) {
        app_client_mock("get", "/lives")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 200,
                "message": null,
                "content": {
                  "data": [
                    {
                      "liveId": 0,
                      "liveTitle": "",
                      "liveThumbnailImageUrl": "",
                      "concurrentUserCount": 0,
                      "openDate": "",
                      "adult": false,
                      "tags": [""],
                      "categoryType": "GAME",
                      "liveCategory": "",
                      "liveCategoryValue": "",
                      "channelId": "",
                      "channelName": "",
                      "channelImageUrl": ""
                    }
                  ],
                  "page": { "next": "" }
                }
            })))
            .mount(&self.server)
            .await;
    }
}

#[derive(Debug)]
pub struct UserHttpMock {
    user_client: tongbal_api::Client,
    server: MockServer,
}

impl UserHttpMock {
    pub async fn new() -> Self {
        let server = MockServer::start().await;
        let mut url_parsed = Url::parse(&server.uri()).unwrap();
        url_parsed.set_path("/open/v1");

        let user_client = tongbal_api::Client::with_client_builder(
            AccessToken::from("FFok65zQFQVcFvH2eJ7SS7SBFlTXt0EZ10L5XXXXXXXX"),
            reqwest::ClientBuilder::new(),
        )
        .with_url(url_parsed);

        Self {
            user_client,
            server,
        }
    }

    pub fn client(&self) -> &tongbal_api::Client {
        &self.user_client
    }
}

impl UserHttpMock {
    pub async fn get_me(&self) {
        user_client_mock("get", "/users/me")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 200,
                "message": null,
                "content": {
                    "channelId": "909501f048b44cf0d5c1d28XXXXXXXX",
                    "channelName": "치지직유저 3121"
                }
            })))
            .mount(&self.server)
            .await
    }
}

impl UserHttpMock {
    pub async fn get_stream_key(&self) {
        user_client_mock("get", "/streams/key")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 200,
                "message": null,
                "content": { "streamKey": "" }
            })))
            .mount(&self.server)
            .await
    }

    pub async fn get_stream_settings(&self) {
        user_client_mock("get", "/lives/setting")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 200,
                "message": null,
                "content": {
                  "defaultLiveTitle": "",
                  "category": null,
                  "tags": []
                }
            })))
            .mount(&self.server)
            .await
    }

    pub async fn set_stream_settings(&self) {
        user_client_mock("patch", "/lives/setting")
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await
    }
}

impl UserHttpMock {
    pub async fn send_message(&self) {
        user_client_mock("post", "/chats/send")
            .and(body_json(json!({
                "message": "hi"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 200,
                "message": null,
                "content": {
                  "messageId": ""
                }
            })))
            .mount(&self.server)
            .await
    }

    pub async fn set_chat_notice(&self) {
        user_client_mock("post", "/chats/notice")
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await
    }

    pub async fn get_chat_settings(&self) {
        user_client_mock("get", "/chats/settings")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 200,
                "message": null,
                "content": {
                  "chatAvailableCondition": "NONE",
                  "chatAvailableGroup": "ALL",
                  "minFollowerMinute": 0,
                  "allowSubscriberInFollowerMode": true
                }
            })))
            .mount(&self.server)
            .await
    }

    pub async fn set_chat_settings(&self) {
        user_client_mock("put", "/chats/settings")
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await
    }
}

fn app_client_mock(http_method: &str, endpoint: &str) -> MockBuilder {
    basic_mock(http_method, endpoint)
        .and(header("client-id", "fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
        .and(header(
            "client-secret",
            "VeIMuc9XGle7PSxIVYNwPpI2OEk_9gXoW_XXXXXXXXX",
        ))
}

fn user_client_mock(http_method: &str, endpoint: &str) -> MockBuilder {
    basic_mock(http_method, endpoint).and(header(
        "authorization",
        "Bearer FFok65zQFQVcFvH2eJ7SS7SBFlTXt0EZ10L5XXXXXXXX",
    ))
}

fn basic_mock(http_method: &str, endpoint: &str) -> MockBuilder {
    Mock::given(method(http_method)).and(path(format!("/open/v1{}", endpoint)))
}
