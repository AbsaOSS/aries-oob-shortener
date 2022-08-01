use serde::{Deserialize, Serialize};

use crate::error::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EcsTaskMetadata {
    container_id: Option<String>,
    container_name: Option<String>,
    created_at: Option<String>,
    started_at: Option<String>,
    cluster_name: Option<String>,
    task_arn: Option<String>,
    task_definition_family: Option<String>,
    task_definition_version: Option<String>,
}

async fn get_ecs_task_metadata(url: &str) -> SResult<reqwest::Response> {
    reqwest::Client::builder()
        .build()?
        .get(url)
        .send()
        .await
        .map_err(|err| err.into())
}

pub async fn fetch_ecs_task_metadata(url: &str) -> SResult<EcsTaskMetadata> {
    let response = get_ecs_task_metadata(url).await?;
    let response_json: serde_json::Value = response.json().await.unwrap();

    let container_id = response_json["DockerId"].as_str().map(String::from);
    let container_name = response_json["Name"].as_str().map(String::from);
    let created_at = response_json["CreatedAt"].as_str().map(String::from);
    let started_at = response_json["StartedAt"].as_str().map(String::from);
    let (cluster_name, task_arn, task_definition_family, task_definition_version) =
        if let Some(labels) = response_json["Labels"].as_object() {
            (
                labels["com.amazonaws.ecs.cluster"]
                    .as_str()
                    .map(String::from),
                labels["com.amazonaws.ecs.task-arn"]
                    .as_str()
                    .map(String::from),
                labels["com.amazonaws.ecs.task-definition-family"]
                    .as_str()
                    .map(String::from),
                labels["com.amazonaws.ecs.task-definition-version"]
                    .as_str()
                    .map(String::from),
            )
        } else {
            (None, None, None, None)
        };

    Ok(EcsTaskMetadata {
        container_id,
        container_name,
        created_at,
        started_at,
        cluster_name,
        task_arn,
        task_definition_family,
        task_definition_version,
    })
}
