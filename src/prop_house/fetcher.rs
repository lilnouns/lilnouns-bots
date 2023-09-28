use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use worker::{Env, Result};

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/prop_house_schema.graphql",
  query_path = "graphql/queries/prop_house_query.graphql",
  response_derives = "Clone",
  skip_serializing_none,
  deprecated = "warn"
)]
struct AuctionQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/prop_house_schema.graphql",
  query_path = "graphql/queries/prop_house_query.graphql",
  response_derives = "Clone",
  skip_serializing_none,
  deprecated = "warn"
)]
struct ProposalQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/prop_house_schema.graphql",
  query_path = "graphql/queries/prop_house_query.graphql",
  response_derives = "Clone, Debug",
  skip_serializing_none,
  deprecated = "warn"
)]
struct VoteQuery;

type DateTime = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct Auction {
  pub id: isize,
  pub title: String,
  pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Proposal {
  pub id: isize,
  pub title: String,
  pub tldr: String,
  pub address: String,
  pub auction_id: isize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vote {
  pub id: isize,
  pub address: String,
  pub auction_id: isize,
  pub proposal_id: isize,
  pub direction: isize,
}

pub struct GraphQLFetcher {
  graphql_url: String,
  community_id: String,
}

impl GraphQLFetcher {
  pub fn new(graphql_url: String, community_id: String) -> Self {
    Self {
      graphql_url,
      community_id,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<GraphQLFetcher> {
    let graphql_url = env.var("PROP_HOUSE_GRAPHQL_URL")?.to_string();
    let community_id = env.var("PROP_HOUSE_COMMUNITY_ID")?.to_string();

    Ok(Self::new(graphql_url, community_id))
  }

  async fn fetch<QueryType: GraphQLQuery>(
    &self,
    variables: <QueryType as GraphQLQuery>::Variables,
  ) -> Option<<QueryType as GraphQLQuery>::ResponseData> {
    let client = Client::builder()
      .build()
      .map_err(|e| {
        error!("Failed to create client: {}", e);
        debug!("Error details: {:?}", e);
      })
      .ok()?;

    post_graphql::<QueryType, _>(&client, &self.graphql_url, variables)
      .await
      .map_err(|e| {
        error!("Failed to execute GraphQL request: {}", e);
        debug!("Failure details: {:?}", e);
      })
      .ok()
      .and_then(|response| response.data)
  }

  pub async fn fetch_auctions(&self) -> Option<Vec<Auction>> {
    let variables = auction_query::Variables {
      id: self.community_id.parse().unwrap(),
    };

    let response = self.fetch::<AuctionQuery>(variables).await?;
    let auctions = response
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

  pub async fn fetch_proposals(&self) -> Option<Vec<Proposal>> {
    let variables = proposal_query::Variables {
      id: self.community_id.parse().unwrap(),
    };

    let response = self.fetch::<ProposalQuery>(variables).await?;

    let proposals = response
      .community
      .auctions
      .iter()
      .flat_map(|auction| {
        auction
          .proposals
          .iter()
          .map(move |proposal| (auction.auction_fragment.id.try_into().unwrap(), proposal))
      })
      .map(|(auction_id, proposal)| Proposal {
        id: proposal.id.try_into().unwrap(),
        title: proposal.title.clone(),
        tldr: proposal.tldr.clone(),
        address: proposal.address.clone(),
        auction_id,
      })
      .collect();

    Some(proposals)
  }

  pub async fn fetch_votes(&self) -> Option<Vec<Vote>> {
    let variables = vote_query::Variables {
      id: self.community_id.parse().unwrap(),
    };

    let response = self.fetch::<VoteQuery>(variables).await?;

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
}
