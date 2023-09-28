use std::convert::TryInto;

use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use worker::{Env, Result};

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/prop_lot_schema.graphql",
  query_path = "graphql/queries/prop_lot_query.graphql",
  response_derives = "Clone",
  deprecated = "warn"
)]
struct IdeaQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/prop_lot_schema.graphql",
  query_path = "graphql/queries/prop_lot_query.graphql",
  response_derives = "Clone",
  deprecated = "warn"
)]
struct VoteQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/prop_lot_schema.graphql",
  query_path = "graphql/queries/prop_lot_query.graphql",
  response_derives = "Clone, Debug",
  deprecated = "warn"
)]
struct CommentQuery;

type Date = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct Idea {
  pub id: isize,
  pub title: String,
  pub tldr: String,
  pub creator_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vote {
  pub id: isize,
  pub voter_id: String,
  pub idea_id: isize,
  pub direction: isize,
  pub voter_weight: isize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
  pub id: isize,
  pub idea_id: isize,
  pub author_id: String,
  pub body: String,
}

pub struct GraphQLFetcher {
  graphql_url: String,
}

impl GraphQLFetcher {
  pub fn new(graphql_url: String) -> Self {
    Self { graphql_url }
  }

  pub fn new_from_env(env: &Env) -> Result<GraphQLFetcher> {
    let graphql_url = env.var("PROP_LOT_GRAPHQL_URL")?.to_string();

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

  pub async fn fetch_ideas(&self) -> Option<Vec<Idea>> {
    let variables = idea_query::Variables {
      options: idea_query::IdeaInputOptions {
        idea_id: None,
        sort: Some(idea_query::SORT_TYPE::OLDEST),
      },
    };

    let response = self.fetch::<IdeaQuery>(variables).await?;

    let ideas = response
      .ideas
      .as_ref()?
      .iter()
      .map(|idea| Idea {
        id: idea.id.try_into().unwrap(),
        title: idea.title.clone(),
        tldr: idea.tldr.clone(),
        creator_id: idea.creator_id.clone(),
      })
      .collect();

    Some(ideas)
  }

  pub async fn fetch_votes(&self) -> Option<Vec<Vote>> {
    let variables = vote_query::Variables {
      options: vote_query::IdeaInputOptions {
        idea_id: None,
        sort: Some(vote_query::SORT_TYPE::LATEST),
      },
    };

    let response = self.fetch::<VoteQuery>(variables).await?;

    let votes = response
      .ideas
      .as_ref()?
      .iter()
      .flat_map(|idea| idea.votes.iter())
      .flat_map(|vote| vote.iter())
      .map(|vote| Vote {
        id: vote.id.try_into().unwrap(),
        voter_id: vote.voter_id.clone(),
        idea_id: vote.idea_id.try_into().unwrap(),
        direction: vote.direction.try_into().unwrap(),
        voter_weight: vote.voter_weight.try_into().unwrap(),
      })
      .collect();

    Some(votes)
  }

  pub async fn fetch_comments(&self) -> Option<Vec<Comment>> {
    let variables = comment_query::Variables {
      options: comment_query::IdeaInputOptions {
        idea_id: None,
        sort: Some(comment_query::SORT_TYPE::LATEST),
      },
    };

    let response = self.fetch::<CommentQuery>(variables).await?;

    let comments = response
      .ideas
      .as_ref()?
      .iter()
      .flat_map(|idea| idea.comments.iter())
      .flat_map(|comment| comment.iter())
      .map(|comment| Comment {
        id: comment.id.try_into().unwrap(),
        idea_id: comment.idea_id.try_into().unwrap(),
        author_id: comment.author_id.clone(),
        body: comment.body.clone(),
      })
      .collect();

    Some(comments)
  }
}
