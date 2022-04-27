use core::time::Duration;
use goose::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

type Headers = HashMap<String, Vec<String>>;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Reflection {
    method: String,
    uri: String,
    version: String,
    headers: Headers,
}

async fn setup_custom_client(user: &mut GooseUser) -> GooseTaskResult {
    let builder = Client::builder()
        .cookie_store(true)
        .gzip(true)
        .timeout(Duration::from_secs(20))
        .danger_accept_invalid_certs(true);
    user.set_client_builder(builder).await?;
    Ok(())
}
async fn loadtest_go(user: &mut GooseUser) -> GooseTaskResult {
    let mut goose = user.get("/go").await?;
    let resp = goose.response?;
    let reflection: Reflection = match resp.json().await {
        Ok(reflection) => reflection,
        Err(e) => return user.set_failure(&format!("{:?}", e), &mut goose.request, None, None),
    };

    if !reflection.headers.contains_key("x-goplugin") {
        return user.set_failure(
            &format!("plugin header missing"),
            &mut goose.request,
            None,
            None,
        );
    }

    Ok(())
}
async fn loadtest_python(user: &mut GooseUser) -> GooseTaskResult {
    let mut goose = user.get("/python").await?;
    let resp = goose.response?;
    let reflection: Reflection = match resp.json().await {
        Ok(reflection) => reflection,
        Err(e) => return user.set_failure(&format!("{:?}", e), &mut goose.request, None, None),
    };

    if !reflection.headers.contains_key("x-pythonplugin") {
        return user.set_failure(
            &format!("plugin header missing"),
            &mut goose.request,
            None,
            None,
        );
    }
    Ok(())
}
async fn loadtest_js(user: &mut GooseUser) -> GooseTaskResult {
    let mut goose = user.get("/js").await?;
    let resp = goose.response?;
    let reflection: Reflection = match resp.json().await {
        Ok(reflection) => reflection,
        Err(e) => return user.set_failure(&format!("{:?}", e), &mut goose.request, None, None),
    };

    if !reflection.headers.contains_key("x-jsplugin") {
        return user.set_failure(
            &format!("plugin header missing"),
            &mut goose.request,
            None,
            None,
        );
    }
    Ok(())
}
async fn loadtest_lua(user: &mut GooseUser) -> GooseTaskResult {
    let mut goose = user.get("/lua").await?;
    let resp = goose.response?;
    let reflection: Reflection = match resp.json().await {
        Ok(reflection) => reflection,
        Err(e) => return user.set_failure(&format!("{:?}", e), &mut goose.request, None, None),
    };

    if !reflection.headers.contains_key("x-luaplugin") {
        return user.set_failure(
            &format!("plugin header missing"),
            &mut goose.request,
            None,
            None,
        );
    }
    Ok(())
}
// async fn loadtest_rust(user: &mut GooseUser) -> GooseTaskResult {
//     let _goose = user.get("/rust").await?;
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    let loadtest_tasks = env::var("LOADTEST_TASKS").unwrap_or("rust,python,go,js,lua".to_string());
    let selected_tasks: Vec<&str> = loadtest_tasks.split(",").collect();

    let mut ts = taskset!("LoadtestTasks").register_task(task!(setup_custom_client).set_on_start());
    if selected_tasks.contains(&"lua") {
        ts = ts.register_task(task!(loadtest_lua));
    }
    if selected_tasks.contains(&"python") {
        ts = ts.register_task(task!(loadtest_python));
    }
    if selected_tasks.contains(&"go") {
        ts = ts.register_task(task!(loadtest_go));
    }
    if selected_tasks.contains(&"js") {
        ts = ts.register_task(task!(loadtest_js));
    }
    GooseAttack::initialize()?
        .register_taskset(ts)
        .set_scheduler(GooseScheduler::Random)
        .execute()
        .await?
        .print();

    Ok(())
}
