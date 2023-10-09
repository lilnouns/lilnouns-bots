use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use log::{debug, error};
use reqwest::Client;
use worker::{Env, Result};

use crate::lil_nouns::{Proposal, Vote};

type Bytes = String;
type BigInt = String;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/lil_nouns_schema.graphql",
  query_path = "graphql/queries/lil_nouns_query.graphql",
  skip_serializing_none,
  deprecated = "warn"
)]
struct ProposalAndVoteQuery;

pub struct GraphQLFetcher {
  graphql_url: String,
}

impl GraphQLFetcher {
  pub fn new(graphql_url: String) -> Self {
    Self { graphql_url }
  }

  pub fn new_from_env(env: &Env) -> Result<GraphQLFetcher> {
    let graphql_url = env.var("LIL_NOUNS_GRAPHQL_URL")?.to_string();

    Ok(Self::new(graphql_url))
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
    let variables = proposal_and_vote_query::Variables {};

    let response = self.fetch::<ProposalAndVoteQuery>(variables).await?;

    let proposals = response
      .proposals
      .iter()
      .map(|proposal| Proposal {
        id: proposal.id.parse::<usize>().unwrap(),
        title: proposal.title.clone(),
        proposer: proposal.proposer.id.clone(),
      })
      .collect();

    Some(proposals)
  }

  pub async fn fetch_votes(&self) -> Option<Vec<Vote>> {
    let variables = proposal_and_vote_query::Variables {};

    let response = self.fetch::<ProposalAndVoteQuery>(variables).await?;

    let votes = response
      .votes
      .iter()
      .map(|vote| Vote {
        id: vote.id.parse::<usize>().unwrap(),
        voter: vote.voter.id.clone(),
        proposal_id: vote.proposal.id.parse::<usize>().unwrap(),
        direction: vote.support_detailed.try_into().unwrap(),
      })
      .collect();

    Some(votes)
  }
}
