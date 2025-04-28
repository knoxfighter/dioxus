#![allow(non_snake_case)]

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use dioxus::logger::tracing::{error, info};
use dioxus::prelude::*;
use dioxus::prelude::server_fn::codec::JsonEncoding;
use dioxus::prelude::server_fn::error::{FromServerFnError, ServerFnErrorErr};

fn main() {
    // Set the url of the server where server functions are hosted.
    #[cfg(not(feature = "server"))]
    dioxus::fullstack::prelude::server_fn::client::set_server_url("http://127.0.0.1:8080");
    dioxus::launch(app);
}

pub fn app() -> Element {
    rsx! {
        button {
            onclick: move |_| async move {
                if let Ok(data) = get_server_data().await {
                    info!("data: {}", data);
                }
            },
            "Success"
        }
        button {
            onclick: move |_| async move {
                if let Err(e) = get_server_error().await {
                    error!("server error: {}", e);
                }
            },
            "Error"
        }
    }
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, CustomError> {
    info!("get_server_data");
    Ok("sucessfull!".to_string())
}

#[server(GetServerError)]
async fn get_server_error() -> Result<String, CustomError> {
    info!("get_server_error");
    Err(CustomError::TestError("error!".to_string()))
}

#[derive(Debug, Serialize, Deserialize)]
enum CustomError {
    TestError(String),
    ServerFnError(ServerFnErrorErr),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromServerFnError for CustomError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self::ServerFnError(value)
    }
}
