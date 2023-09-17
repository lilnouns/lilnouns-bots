use std::env;
use std::time::Duration;

use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/prop_house_schema.graphql",
    query_path = "graphql/queries/prop_house_query.graphql",
    response_derives = "Clone",
    deprecated = "warn"
)]
struct AuctionQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/prop_house_schema.graphql",
    query_path = "graphql/queries/prop_house_query.graphql",
    response_derives = "Clone",
    deprecated = "warn"
)]
struct ProposalQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/prop_house_schema.graphql",
    query_path = "graphql/queries/prop_house_query.graphql",
    response_derives = "Clone",
    deprecated = "warn"
)]
struct VoteQuery;

type DateTime = String;

#[derive(Serialize, Deserialize)]
pub(crate) struct Auction {
    pub(crate) id: isize,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Proposal {
    pub(crate) id: isize,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Vote {
    pub(crate) id: isize,
}

pub async fn fetch_auctions() -> Option<Vec<Auction>> {
    let url = env::var("PROP_HOUSE_GRAPHQL_URL")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let community_id = env::var("PROP_HOUSE_COMMUNITY_ID")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let variables = auction_query::Variables {
        id: community_id.parse().unwrap(),
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create client: {}", e);
        })
        .ok()?;

    let response = post_graphql::<AuctionQuery, _>(&client, url, variables)
        .await
        .map_err(|e| {
            error!("Failed to execute GraphQL request: {}", e);
        })
        .ok()?;

    let auctions = response
        .data
        .as_ref()?
        .community
        .auctions
        .iter()
        .map(|auction| Auction {
            id: auction.id.try_into().unwrap(),
            title: auction.title.clone(),
            description: auction.description.clone(),
        })
        .collect();

    Some(auctions)
}

pub async fn fetch_proposals() -> Option<Vec<Proposal>> {
    let url = env::var("PROP_HOUSE_GRAPHQL_URL")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let community_id = env::var("PROP_HOUSE_COMMUNITY_ID")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let variables = proposal_query::Variables {
        id: community_id.parse().unwrap(),
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create client: {}", e);
        })
        .ok()?;

    let response = post_graphql::<ProposalQuery, _>(&client, url, variables)
        .await
        .map_err(|e| {
            error!("Failed to execute GraphQL request: {}", e);
        })
        .ok()?;

    let proposals = response
        .data
        .as_ref()?
        .community
        .auctions
        .iter()
        .flat_map(|auction| &auction.proposals)
        .map(|proposal| Proposal {
            id: proposal.id.try_into().unwrap(),
        })
        .collect();

    Some(proposals)
}

pub async fn fetch_votes() -> Option<Vec<Vote>> {
    let url = env::var("PROP_HOUSE_GRAPHQL_URL")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let community_id = env::var("PROP_HOUSE_COMMUNITY_ID")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let variables = vote_query::Variables {
        id: community_id.parse().unwrap(),
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create client: {}", e);
        })
        .ok()?;

    let response = post_graphql::<VoteQuery, _>(&client, url, variables)
        .await
        .map_err(|e| {
            error!("Failed to execute GraphQL request: {}", e);
        })
        .ok()?;

    let votes = response
        .data
        .as_ref()?
        .community
        .auctions
        .iter()
        .flat_map(|auction| &auction.proposals)
        .flat_map(|proposal| &proposal.votes)
        .map(|vote| Vote {
            id: vote.id.try_into().unwrap(),
        })
        .collect();

    Some(votes)
}
