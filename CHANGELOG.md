# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0-alpha.13] - 2023-09-21

### Miscellaneous Tasks

- Move `kv_namespaces` to `env.dev` in `wrangler.toml`

## [1.0.0-alpha.12] - 2023-09-21

### Refactor

- Update key names for prop lot and prop house setup
- Improve cache handling for ideas, votes, comments
- Add info logs in Prop Lot and Prop House setup checks
- Update caching strategy in prop house handler
- Update to fetch all ideas in vote and comment handlers
- Improve caching and fetching logic in prop house module
- Improve logging for fetch and handler operations
- Add logging to DiscordHandler operations
- Update data type in prop lot module

### Miscellaneous Tasks

- Add `dev` environment in `wrangler.toml`
- Remove redundant worker-build steps from deploy workflow
- Update API token reference in deployment workflow
- Add KV namespace binding to wrangler.toml

## [1.0.0-alpha.11] - 2023-09-21

### Miscellaneous Tasks

- Update deployment settings in GitHub Actions
- Add cron `triggers` to `wrangler.toml`

## [1.0.0-alpha.10] - 2023-09-21

### Refactor

- Improve error handling for Prop Lot and Prop House setup

### Miscellaneous Tasks

- Update triggers for Deploy & Publish Workflow

## [1.0.0-alpha.9] - 2023-09-21

### Refactor

- Update Cache struct to hold KvStore directly
- Update DiscordHandler and GraphQLFetcher to remove lifetime notations

### Miscellaneous Tasks

- Move secret environment variables to deploy workflow
- Add PNPM dependencies maintenance to dependabot
- Optimize worker-build installation in GitHub workflows

## [1.0.0-alpha.8] - 2023-09-21

### Bug Fixes

- Replace incorrect field mapping in Prop House comment model

### Refactor

- Remove caching system from project
- Update fetcher functions to use worker env
- Update fetcher to use GraphQLFetcher struct
- Add caching functionality to enhance performance
- Move code into `lib.rs` for modularity and error handling
- Update caching and fetching mechanism
- Update the `pnpm-lock.yaml` and `Cargo.lock` files
- Update DiscordHandler to use a custom in-memory cache system
- Update formatting and date variable in prop handlers

### Miscellaneous Tasks

- Add `package.json` for LilNouns bots project
- Add `wrangler.toml` configuration for deployment
- Add `wasm-pack` and additional dependencies in `Cargo.toml`
- Add `node_modules` to `.gitignore`
- Update `package.json` for deployment and development
- Add `wasm32` target to Cargo configuration
- Update and rearrange dependencies
- Update `dev` script in `package.json`
- Add `.wrangler` to `.gitignore`
- Update build process in GitHub Actions workflow
- Update build command in GitHub Actions workflow
- Add deployment workflow for GitHub Actions

## [1.0.0-alpha.7] - 2023-09-19

### Refactor

- Update cache set and get operations for efficiency
- Update caching functions to use map and collect methods
- Update notification messages for clarity and tracking

## [1.0.0-alpha.6] - 2023-09-18

### Refactor

- Add new fields to Proposal and Vote in Prop House fetcher
- Update notification handling for new Prop House proposals and votes
- Move handler functions to DiscordHandler structs
- Convert Prop House auction description from HTML to Markdown
- Change default sorting in Prop Lot fetcher to oldest
- Add author id and body to Prop Lot comment struct
- Update vote and comment Prop Lot handler methods

### Miscellaneous Tasks

- Add auction field to Prop House query

## [1.0.0-alpha.5] - 2023-09-18

### Refactor

- Update code to handle new proposals and votes
- Add fetch functions to reduce code duplication
- Update codebase to use generic caching system
- Add proposal title and link votes to proposals for Prop House
- Improve proposal and vote logging on Prop House
- Improve cache flexibility and performance
- Add batch insertion capability to improve caching performance
- Switch storage engine from `sled` to `rocksdb`
- Update caching process and improve logging
- Update async code for clarity and simplicity

### Miscellaneous Tasks

- Remove `sled` dependency from `Cargo.toml`
- Add report files to `.gitignore`
- Update dependencies and remove unneeded ones

## [1.0.0-alpha.4] - 2023-09-17

### Refactor

- Remove unnecessary clone method and handle string conversion
- Limit visibility of fetch and cache functions

### Miscellaneous Tasks

- Add caching to GitHub Actions workflow

## [1.0.0-alpha.3] - 2023-09-17

### Refactor

- Update Prop Lot Query struct and GraphQL query for simplicity
- Change `id` data type from `i32` to `isize` and refactor Idea
- Add vote fetching functionality to Prop Lot fetcher module
- Add comments fetching in Prop Lot module
- Add caching for votes and comments for Prop Lot
- Simplify async result handling in `setup` and `start` for Prop Lot
- Add caching for votes and comments of Prop Lot
- Update Prop House query to increase maintainability
- Update Prop House fetcher for granular queries
- Add caching for proposals and votes

### Documentation

- Add status badges to README

### Miscellaneous Tasks

- Update `Cargo.toml` to include `.graphql` files
- Add VoteFragment and VoteQuery to Prop Lot query

## [1.0.0-alpha.2] - 2023-09-15

### Documentation

- Add `README.md` for project description

### Miscellaneous Tasks

- Update test execution tool in Github workflow

## [1.0.0-alpha.1] - 2023-09-15

### Documentation

- Add issue templates for bug report and feature request
- Add `FUNDING.yml` for sponsorship information

### Miscellaneous Tasks

- Add dependabot configuration for GitHub Actions and Cargo
- Add stale bot configuration
- Add GitHub `build` workflow with concurrency and auto-release

## [1.0.0-alpha.0] - 2023-09-15

### Features

- Implement global caching functionality
- Add caching functionality for fetched auctions and ideas
- Add command line interface to manage setup process
- Add `handler` module to `prop_house` and `prop_lot`
- Add handling for new ideas in `prop_lot` module
- Add `prop_house` auction handling and cache optimisation

### Refactor

- Update `main` function to fetch ideas from Prop Lot GraphQL API
- Enhance error handling in `fetch_prop_lot_ideas` function
- Improve and modularize Prop Lot fetching logic
- Add Prop House fetcher module
- Add `prop_house` module and fetch auction Ids
- Add Cache implementation for data persistence
- Add 30 seconds timeout for GraphQL requests
- Update main loop, centralize setup logic
- Improve error handling in fetchers and setup functions
- Improve `prop_lot` setup function to simplify async call
- Improve `prop_house` setup function to simplify error handling
- Add logging and start command to main
- Add functionality to handle new auctions in Prop House
- Add functionality to handle new ideas in Prop Lot
- Improve error handling in Cache module
- Add error logging for idea and auction handling failures
- Improve error handling in cache setting
- Add author display to idea creation message

### Documentation

- Add new LICENSE file for repository (#1)

### Miscellaneous Tasks

- Initialize Rust project for `lilnouns-bots`
- Add GraphQL schema for Prop Lot
- Add the PropLot query to GraphQL
- Create GraphQL schema for Prop House
- Add new Prop House query in GraphQL
- Add `tmp` to `.gitignore`
- Add `git-cliff` configuration and initial CHANGELOG.md

### Styling

- Update formatting for code readability

<!-- generated by git-cliff -->
