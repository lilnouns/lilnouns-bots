use std::env;
use std::time::Duration;

use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/prop_lot_schema.graphql",
    query_path = "graphql/queries/prop_lot_query.graphql",
    response_derives = "Clone",
    deprecated = "warn"
)]
struct IdeaQuery;

#[derive(Serialize, Deserialize)]
pub(crate) struct Idea {
    pub(crate) id: isize,
    pub(crate) title: String,
    pub(crate) tldr: String,
    pub(crate) creator_id: String,
}

pub async fn fetch_ideas() -> Option<Vec<Idea>> {
    let url = env::var("PROP_LOT_GRAPHQL_URL")
        .map_err(|_| {
            error!("PROP_LOT_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let variables = idea_query::Variables {
        options: idea_query::IdeaInputOptions {
            idea_id: None,
            sort: Some(idea_query::SORT_TYPE::LATEST),
        },
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create client: {}", e);
        })
        .ok()?;

    let response = post_graphql::<IdeaQuery, _>(&client, url, variables)
        .await
        .map_err(|e| {
            error!("Failed to execute GraphQL request: {}", e);
        })
        .ok()?;

    let ideas = response
        .data
        .as_ref()?
        .ideas
        .as_ref()?
        .into_iter()
        .map(|idea| Idea {
            id: idea.id.try_into().unwrap(),
            title: idea.title.clone(),
            tldr: idea.tldr.clone(),
            creator_id: idea.creator_id.clone(),
        })
        .collect();

    Some(ideas)
}
