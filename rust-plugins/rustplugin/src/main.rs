// src/main.rs
use kong_rust_pdk::{macros::*, pdk::Pdk, server, Error, Plugin};

const VERSION: &str = "0.1";
const PRIORITY: usize = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::start::<Config>(VERSION, PRIORITY).await?;

    Ok(())
}

#[plugin_config]
struct Config {
    message: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            message: String::from("default message"),
        }
    }
}

#[plugin_impl]
impl Plugin for Config {
    async fn access<T: Pdk>(&self, kong: &mut T) -> Result<(), Error> {
        let method = kong.request().get_method().await?;

        kong.response().set_status(204).await?;

        kong.response()
            .set_header("x-hello-from-rust", &method)
            .await?;
        // kong.service_request()
        //     .set_header("x-rustplugin", "Foobar")
        //     .await?;

        Ok(())
    }
}
