use std::env;
use std::time::Duration;

use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use log::error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/prop_house_schema.graphql",
    query_path = "graphql/queries/prop_house_query.graphql",
    request_derives = "Debug",
    response_derives = "Debug, Serialize, Deserialize, Clone",
    variables_derives = "Debug",
    deprecated = "warn"
)]
struct Query;

type DateTime = String;

pub(crate) type Auction = query::QueryCommunityAuctions;

pub async fn fetch_auctions() -> Option<Vec<Auction>> {
    let url = env::var("PROP_HOUSE_GRAPHQL_URL").ok();

    if url.is_none() {
        error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        return None;
    }

    let variables = query::Variables { id: 2 };

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

    response.data.map_or_else(
        || {
            error!("Response data is unavailable");
            None
        },
        |data| Some(data.community.auctions),
    )
}
