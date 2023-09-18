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

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Proposal {
    pub(crate) id: isize,
    pub(crate) title: String,
    pub(crate) tldr: String,
    pub(crate) address: String,
    pub(crate) auction_id: isize,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Vote {
    pub(crate) id: isize,
    pub(crate) address: String,
    pub(crate) auction_id: isize,
    pub(crate) proposal_id: isize,
    pub(crate) direction: isize,
}

async fn fetch<QueryType: GraphQLQuery>(
    variables: <QueryType as GraphQLQuery>::Variables,
) -> Option<<QueryType as GraphQLQuery>::ResponseData> {
    let url = env::var("PROP_HOUSE_GRAPHQL_URL")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create client: {}", e);
        })
        .ok()?;

    post_graphql::<QueryType, _>(&client, url, variables)
        .await
        .map_err(|e| {
            error!("Failed to execute GraphQL request: {}", e);
        })
        .ok()
        .and_then(|response| response.data)
}

pub(crate) async fn fetch_auctions() -> Option<Vec<Auction>> {
    let community_id = env::var("PROP_HOUSE_COMMUNITY_ID")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let variables = auction_query::Variables {
        id: community_id.parse().unwrap(),
    };

    let response = fetch::<AuctionQuery>(variables).await?;

    let auctions = response
        .community
        .auctions
        .iter()
        .map(|auction| Auction {
            id: auction.id.try_into().unwrap(),
            title: auction.title.clone(),
            description: html2md::parse_html(&*auction.description),
        })
        .collect();

    Some(auctions)
}

pub(crate) async fn fetch_proposals() -> Option<Vec<Proposal>> {
    let community_id = env::var("PROP_HOUSE_COMMUNITY_ID")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let variables = proposal_query::Variables {
        id: community_id.parse().unwrap(),
    };

    let response = fetch::<ProposalQuery>(variables).await?;

    let proposals = response
        .community
        .auctions
        .iter()
        .flat_map(|auction| &auction.proposals)
        .map(|proposal| Proposal {
            id: proposal.id.try_into().unwrap(),
            title: proposal.title.clone(),
            tldr: proposal.tldr.clone(),
            address: proposal.address.clone(),
            auction_id: proposal.auction.id.try_into().unwrap(),
        })
        .collect();

    Some(proposals)
}

pub(crate) async fn fetch_votes() -> Option<Vec<Vote>> {
    let community_id = env::var("PROP_HOUSE_COMMUNITY_ID")
        .map_err(|_| {
            error!("PROP_HOUSE_GRAPHQL_URL is not set in env");
        })
        .ok()?;

    let variables = vote_query::Variables {
        id: community_id.parse().unwrap(),
    };

    let response = fetch::<VoteQuery>(variables).await?;

    let votes = response
        .community
        .auctions
        .iter()
        .flat_map(|auction| &auction.proposals)
        .flat_map(|proposal| &proposal.votes)
        .map(|vote| Vote {
            id: vote.id.try_into().unwrap(),
            address: vote.address.clone(),
            auction_id: vote.auction_id.try_into().unwrap(),
            proposal_id: vote.proposal_id.try_into().unwrap(),
            direction: vote.direction.try_into().unwrap(),
        })
        .collect();

    Some(votes)
}
