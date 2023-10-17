use chrono::{Duration, Utc};
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::Client;
use worker::{console_debug as debug, console_error as error, Env, Result};

use crate::meta_gov::{Proposal, Vote};

type Any = i32;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/snapshot_schema.graphql",
  query_path = "graphql/queries/snapshot_query.graphql",
  response_derives = "Clone, Debug",
  skip_serializing_none,
  deprecated = "warn"
)]
struct ProposalQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/snapshot_schema.graphql",
  query_path = "graphql/queries/snapshot_query.graphql",
  response_derives = "Clone, Debug",
  skip_serializing_none,
  deprecated = "warn"
)]
struct VoteQuery;

pub struct GraphQLFetcher {
  graphql_url: String,
  space_id: String,
}

impl GraphQLFetcher {
  pub fn new(graphql_url: String, space_id: String) -> Self {
    Self {
      graphql_url,
      space_id,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<GraphQLFetcher> {
    let graphql_url = env.var("META_GOV_SNAPSHOT_GRAPHQL_URL")?.to_string();
    let space_id = env.var("META_GOV_SNAPSHOT_SPACE_ID")?.to_string();

    Ok(Self::new(graphql_url, space_id))
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

  pub async fn fetch_proposals(&self) -> Option<Vec<Proposal>> {
    let variables = proposal_query::Variables {
      space: Some(self.space_id.clone()),
    };

    let response = self.fetch::<ProposalQuery>(variables).await?;

    let proposals = response
      .proposals
      .as_ref()?
      .into_iter()
      .filter_map(|proposal| proposal.as_ref())
      .map(|proposal| Proposal {
        id: proposal.id.to_string(),
        title: proposal.title.to_string(),
        body: proposal.body.clone().unwrap(),
      })
      .collect();

    Some(proposals)
  }

  pub async fn fetch_votes(&self) -> Option<Vec<Vote>> {
    let now = Utc::now();
    let thirty_days_ago = now - Duration::days(30);

    let variables = vote_query::Variables {
      space: Some(self.space_id.clone()),
      created_gt: thirty_days_ago.timestamp().try_into().unwrap(),
    };

    let response = self.fetch::<VoteQuery>(variables).await?;

    let votes = response
      .votes
      .as_ref()?
      .iter()
      .filter_map(|vote_option| vote_option.as_ref())
      .map(|vote| Vote {
        id: vote.id.to_string(),
        voter: vote.voter.to_string(),
        choice: vote.choice.try_into().unwrap(),
        proposal_id: vote.proposal.clone().unwrap().id,
      })
      .collect();

    Some(votes)
  }
}
