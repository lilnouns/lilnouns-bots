use std::convert::TryInto;
use std::env;
use std::time::Duration;

use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
    response_derives = "Clone",
    deprecated = "warn"
)]
struct CommentQuery;

type Date = String;

#[derive(Serialize, Deserialize)]
pub(crate) struct Idea {
    pub(crate) id: isize,
    pub(crate) title: String,
    pub(crate) tldr: String,
    pub(crate) creator_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Vote {
    pub(crate) id: isize,
    pub(crate) voter_id: String,
    pub(crate) idea_id: isize,
    pub(crate) direction: isize,
    pub(crate) voter_weight: isize,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Comment {
    pub(crate) id: isize,
}

async fn fetch<QueryType: GraphQLQuery>(
    variables: <QueryType as GraphQLQuery>::Variables,
) -> Option<<QueryType as GraphQLQuery>::ResponseData> {
    let url = env::var("PROP_LOT_GRAPHQL_URL")
        .map_err(|_| {
            error!("PROP_LOT_GRAPHQL_URL is not set in env");
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

pub(crate) async fn fetch_ideas() -> Option<Vec<Idea>> {
    let variables = idea_query::Variables {
        options: idea_query::IdeaInputOptions {
            idea_id: None,
            sort: Some(idea_query::SORT_TYPE::LATEST),
        },
    };

    let response = fetch::<IdeaQuery>(variables).await?;

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

pub(crate) async fn fetch_votes() -> Option<Vec<Vote>> {
    let variables = vote_query::Variables {
        options: vote_query::IdeaInputOptions {
            idea_id: None,
            sort: Some(vote_query::SORT_TYPE::LATEST),
        },
    };

    let response = fetch::<VoteQuery>(variables).await?;

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

pub(crate) async fn fetch_comments() -> Option<Vec<Comment>> {
    let variables = comment_query::Variables {
        options: comment_query::IdeaInputOptions {
            idea_id: None,
            sort: Some(comment_query::SORT_TYPE::LATEST),
        },
    };

    let response = fetch::<CommentQuery>(variables).await?;

    let comments = response
        .ideas
        .as_ref()?
        .iter()
        .flat_map(|idea| idea.comments.iter())
        .flat_map(|comment| comment.iter())
        .map(|comment| Comment {
            id: comment.id.try_into().unwrap(),
        })
        .collect();

    Some(comments)
}
