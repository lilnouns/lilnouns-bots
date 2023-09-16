use std::env;
use std::time::Duration;

use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use log::error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/prop_lot_schema.graphql",
    query_path = "graphql/queries/prop_lot_query.graphql",
    request_derives = "Debug",
    response_derives = "Debug, Serialize, Deserialize, Clone",
    variables_derives = "Debug",
    deprecated = "warn"
)]
struct IdeaQuery;

type Date = String;
pub(crate) type Idea = query::QueryIdeas;

pub async fn fetch_ideas() -> Option<Vec<Idea>> {
    let url = env::var("PROP_LOT_GRAPHQL_URL").ok();

    if url.is_none() {
        error!("PROP_LOT_GRAPHQL_URL is not set in env");
        return None;
    }

    let variables = query::Variables {
        options: query::IdeaInputOptions {
            idea_id: None,
            sort: Some(query::SORT_TYPE::LATEST),
        },
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create client: {}", e);
        })
        .ok()?;

    let response = post_graphql::<Query, _>(&client, url.unwrap().to_string(), variables)
        .await
        .map_err(|e| {
            error!("Failed to execute GraphQL request: {}", e);
        })
        .ok()?;

    let ideas = response
        .data
        .and_then(|data| {
            data.ideas.map_or_else(
                || {
                    error!("Ideas not found in the response data");
                    None
                },
                |ideas| Some(ideas),
            )
        })
        .or_else(|| {
            error!("Response data is unavailable");
            None
        });

    ideas
}
