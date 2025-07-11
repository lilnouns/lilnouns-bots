# Changelog

All notable changes to this project will be documented in this file.

## [1.3.0-alpha.23] - 2025-07-02

### Bug Fixes

- Improve `vote.reason` truncation logic

## [1.3.0-alpha.22] - 2025-06-28

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.21] - 2025-06-21

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.20] - 2025-06-06

### Bug Fixes

- Update `WARPCAST_API_BASE_URL` to new endpoint

## [1.3.0-alpha.19] - 2025-04-05

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.18] - 2025-03-24

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.17] - 2025-03-15

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.16] - 2025-02-27

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.15] - 2025-02-20

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.14] - 2025-01-24

### Refactor

- Update `tracing_subscriber` configuration

### Styling

- Reorder imports for better readability
- Reorder imports for better readability

## [1.3.0-alpha.13] - 2025-01-24

### Refactor

- Replace `worker_logger` with `tracing_subscriber`

## [1.3.0-alpha.12] - 2025-01-23

### Miscellaneous Tasks

- Update logger initialization to new library

## [1.3.0-alpha.11] - 2025-01-23

### Refactor

- Use `wasm_logger` for better logging

## [1.3.0-alpha.10] - 2025-01-21

### Miscellaneous Tasks

- Remove explicit `pnpm` version input

## [1.3.0-alpha.9] - 2025-01-21

### Miscellaneous Tasks

- Disable `LIL_NOUNS_DISCORD_ENABLED`

## [1.3.0-alpha.8] - 2025-01-21

### Miscellaneous Tasks

- Enable observability in `wrangler.toml`

## [1.3.0-alpha.7] - 2025-01-20

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.6] - 2025-01-10

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.5] - 2025-01-04

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.4] - 2024-12-25

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.3] - 2024-12-11

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.2] - 2024-12-04

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.3.0-alpha.1] - 2024-11-24

### Refactor

- Replace `unwrap` with `?` operator

### Miscellaneous Tasks

- Update logging setup

## [1.3.0-alpha.0] - 2024-10-10

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.2.0] - 2024-10-10

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.2.0-alpha.2] - 2024-10-10

### Features

- Add required arguments and new fields for snapshot

## [1.2.0-alpha.1] - 2024-09-27

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.2.0-alpha.0] - 2024-09-19

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.1.0] - 2024-09-19

### Miscellaneous Tasks

- Update governance and proposal settings

## [1.1.0-alpha.57] - 2024-09-01

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.1.0-alpha.56] - 2024-08-10

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.1.0-alpha.55] - 2024-08-03

### Miscellaneous Tasks

- Update deps `LIL_NOUNS_GRAPHQL_URL`

## [1.1.0-alpha.54] - 2024-07-30

### Miscellaneous Tasks

- Enable `META_GOV_FARCASTER`

## [1.1.0-alpha.53] - 2024-07-26

### Documentation

- Update `README.md` for clarity

## [1.1.0-alpha.52] - 2024-07-26

### Features

- Add `castDistribution` field to request data

### Miscellaneous Tasks

- Update `LIL_NOUNS_BASE_URL` to new endpoint
- Disable `META_GOV_FARCASTER_ENABLED`
- Disable `PROP_HOUSE_ENABLED` flag
- Disable `PROP_LOT_ENABLED`

## [1.1.0-alpha.51] - 2024-07-07

### Refactor

- Change address format to hexadecimal in farcaster

## [1.1.0-alpha.50] - 2024-07-04

### Features

- Add `ethereum` module for transaction data retrieval
- Add function to retrieve transaction signer
- Enhance proposal data extraction in met gov

### Refactor

- Modify error handling in transaction data function
- Ensure `get_transaction_signer` returns valid address
- Rearrange import order and unwrap signer in farcaster
- Handle errors in getting transaction signer
- Update order of imports and modify wallet logic in farcaster

### Miscellaneous Tasks

- Add new queries and types for Nouns auction & voting system
- Add proposal and vote query
- Add `ProposalQuery` to graphql queries

## [1.1.0-alpha.49] - 2024-06-18

### Features

- Trim comment and vote reasons in farcaster handlers

## [1.1.0-alpha.48] - 2024-06-18

### Features

- Increase character limit for comments and vote reasons

### Miscellaneous Tasks

- Create new `FUNDING.json` (#197)

## [1.1.0-alpha.47] - 2024-06-02

### Refactor

- Update collection URL in farcaster handler

## [1.1.0-alpha.46] - 2024-05-23

### Revert

- Update warpcast channel for various services

## [1.1.0-alpha.45] - 2024-05-20

### Miscellaneous Tasks

- Update warpcast channel for various services

## [1.1.0-alpha.44] - 2024-05-08

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.1.0-alpha.43] - 2024-04-23

### Revert

- Enable smart placment on `wrangler.toml`

## [1.1.0-alpha.42] - 2024-04-23

### Features

- Add a new function to get farcaster names

### Refactor

- Update all related variables related to  Warpcast
- Replace `get_wallet_handle` by `get_username_by_address`

### Miscellaneous Tasks

- Enable smart placment on `wrangler.toml`

## [1.1.0-alpha.41] - 2024-04-21

### Bug Fixes

- Solve issue related to second market schedule

## [1.1.0-alpha.40] - 2024-04-19

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.1.0-alpha.39] - 2024-04-05

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.1.0-alpha.38] - 2024-04-05

### Refactor

- Move second market bot to daily schedule

## [1.1.0-alpha.37] - 2024-03-26

### Refactor

- Replace `day` by `try_day` on metgov fetcher

## [1.1.0-alpha.36] - 2024-03-21

### Bug Fixes

- Solve some minor issues and update dependencies
- Solve some minor issues and update dependencies

## [1.1.0-alpha.35] - 2024-03-04

### Miscellaneous Tasks

- Change bot channels to the new channel

## [1.1.0-alpha.34] - 2024-02-23

### Miscellaneous Tasks

- Change proplot bot farcaster channel to default

## [1.1.0-alpha.33] - 2024-02-21

### Miscellaneous Tasks

- Update farcaster bot channels back to the default

## [1.1.0-alpha.32] - 2024-02-21

### Miscellaneous Tasks

- Move back secondery market bot to the old channel

## [1.1.0-alpha.31] - 2024-02-21

### Miscellaneous Tasks

- Rename the farcaser channel for bots

## [1.1.0-alpha.30] - 2024-02-09

### Revert

- Disable most of farcaster bots on development
- Solve embed url issue in second market farcaster handler
- Replace second market OpenSea urls by token EIP155 link
- Make second market bot to only run at midnight
- Update second market farcaster handler to remove unused codes

## [1.1.0-alpha.29] - 2024-02-08

### Refactor

- Update second market farcaster handler to remove unused codes

### Miscellaneous Tasks

- Disable most of farcaster bots on development

## [1.1.0-alpha.28] - 2024-02-04

### Bug Fixes

- Solve embed url issue in second market farcaster handler

## [1.1.0-alpha.27] - 2024-01-30

### Refactor

- Replace second market OpenSea urls by token EIP155 link

### Miscellaneous Tasks

- Add new git flow workflow for handling pull requests

## [1.1.0-alpha.26] - 2024-01-20

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.1.0-alpha.25] - 2024-01-11

### Bug Fixes

- Make second market bot to only run at midnight

### Documentation

- Add a new badge for wakatime to the `README.md`

## [1.1.0-alpha.24] - 2024-01-07

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.1.0-alpha.23] - 2024-01-03

### Documentation

- Add new badge for Farcaster on project readme file

### Miscellaneous Tasks

- Remove Licensebot configuration file

### Revert

- Update GitHuB Actions versions on build workflow
- Add a new job to the build workflow for analyse codes

## [1.1.0-alpha.22] - 2024-01-01

### Miscellaneous Tasks

- Update GitHuB Actions versions on build workflow

## [1.1.0-alpha.21] - 2024-01-01

### Bug Fixes

- Update second market floor after handle new floor

### Miscellaneous Tasks

- Add a new job to the build workflow for analyse codes

## [1.1.0-alpha.20] - 2023-12-25

### Refactor

- Remove `get_final_url` from utilities
- Replace floors by collections for handling floors

### Miscellaneous Tasks

- Disable all bots except second market ones

### Revert

- Disable all bots except second market ones

## [1.1.0-alpha.19] - 2023-12-15

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.1.0-alpha.18] - 2023-12-05

### Refactor

- Update URL formats for OpenSea in discord and farcaster second market handlers

## [1.1.0-alpha.17] - 2023-12-05

### Refactor

- Simplify condition for handling new floor price

## [1.1.0-alpha.16] - 2023-11-29

### Refactor

- Add URL generation with final URL utility function

## [1.1.0-alpha.15] - 2023-11-28

### Bug Fixes

- Update cache handling in second market module

## [1.1.0-alpha.14] - 2023-11-26

### Refactor

- Add timestamp to floor price update URL
- Change DateTime format to Unix timestamp for links

## [1.1.0-alpha.13] - 2023-11-26

### Refactor

- Add some comments to the second market module
- Improve code for handling floor prices in Second Market
- Update Floor struct and adapt price handling code
- Remove `Link` dependency from `FarcasterHandler` on Prop House
- Include vote reason in vote struct and description of Meta Gov
- Update indentation in farcaster handler for Prop Lot
- Handle default values for new and old prices in second market
- Handle null prices in second market handlers

## [1.1.0-alpha.12] - 2023-11-24

### Bug Fixes

- Update floor price change notification text

## [1.1.0-alpha.11] - 2023-11-20

### Revert

- Update date format to use Utc timestamp across discord handlers

## [1.1.0-alpha.10] - 2023-11-19

### Refactor

- Update date format to use Utc timestamp across discord handlers

## [1.1.0-alpha.9] - 2023-11-19

### Refactor

- Update avatar URLs in Discord webhook handlers
- Format floor price changes in Discord notifications
- Add collection support to handlers of second market
- Update OpenSea url format in handlers of second market

## [1.1.0-alpha.8] - 2023-11-19

### Refactor

- Update floor price change message format in discord handler
- Simplify URL assignment in second market discord handler
- Implement farcaster handler to handle new second market floor

## [1.1.0-alpha.7] - 2023-11-18

### Miscellaneous Tasks

- Add Second Market Discord and Warp Cast tokens
- Add second market settings to `wrangler.toml`

## [1.1.0-alpha.6] - 2023-11-18

### Features

- Add the second market module and debug function

### Refactor

- Add Floor structure to second market module
- Update fetch floors function to return data
- Add new second market structure and handlers
- Add logging and setup for second market module
- Add extended logging and floor management in second market
- Update data structures and fetcher for second market
- Implement handlers and update fetch floors function
- Update Floor struct and handling to incorporate `kind` field
- Add Discord integration to second market handler
- Add debug and warning logs in second market module
- Update URL order and enhance floor price display

### Miscellaneous Tasks

- Disable all feature settings in `wrangler.toml`
- Updated `.dev.vars.example` with Second Market settings
- Add `SECOND_MARKET_API_KEY` to deployment workflow
- Add new second market settings in `wrangler.toml`

### Revert

- Disable all feature settings in `wrangler.toml`

## [1.1.0-alpha.5] - 2023-11-11

### Refactor

- Improve error logging for resolving domain fields

### Miscellaneous Tasks

- Updated the version in `Cargo.lock`

## [1.1.0-alpha.4] - 2023-11-03

### Refactor

- Update reason handling in vote system

## [1.1.0-alpha.3] - 2023-10-30

### Bug Fixes

- Handle potential None case for Vote reason on lil nouns

### Miscellaneous Tasks

- Change warpcast channel in config settings

## [1.1.0-alpha.2] - 2023-10-29

### Features

- Add voting reasons to fetched and handled data for lil nouns

### Refactor

- Add error handling for failed cache storages
- Serialize idea cast map before caching
- Change key type for idea_casts in farcaster
- Update cron schedule in `wrangler.toml`
- Changed caching method for proposal casts to string
- Update cron schedule in `wrangler.toml`

## [1.1.0-alpha.1] - 2023-10-25

### Bug Fixes

- Update Any type and vote choice conversion

### Refactor

- Update warpcast channel for lil nouns
- Replace unwrap calls with detailed error messages
- Improve error handling for `cast_hash` retrieval on prop lot farcaster
- Add debug logs for proposal and idea cast handling
- Handle null proposal scenario in discord handler
- Update warp cast channel in Prop House and Prop Lot configs

## [1.1.0-alpha.0] - 2023-10-19

### Refactor

- Add `warpcast_url` to `FarcasterHandler` in all modules
- Extract proposal logic in meta gov farcaster

### Miscellaneous Tasks

- Update warpcast channel for meta gov to nouns

## [1.0.4-alpha.0] - 2023-10-18

### Refactor

- Change proposal and idea ids to string in caches
- Replace unwrap with error handling in farcaster handlers
- Remove unnecessary condition check before HTTP request
- Replace `unwrap_or` with `ok_or` in retrieving hash

### Miscellaneous Tasks

- Update crons schedule in `wrangler.toml`

## [1.0.3] - 2023-10-18

### Bug Fixes

- Solve some minor issues and update dependencies

### Refactor

- Update discord webhook bot names and avatars

## [1.0.2] - 2023-10-18

### Bug Fixes

- Solve some minor issues and update dependencies

## [1.0.2-alpha.3] - 2023-10-18

### Bug Fixes

- Add avatar URL to webhook execution for discord

## [1.0.2-alpha.2] - 2023-10-18

### Refactor

- Simplify bot usernames in webhook executions

## [1.0.2-alpha.1] - 2023-10-18

### Miscellaneous Tasks

- Update deploy command in workflow

## [1.0.2-alpha.0] - 2023-10-18

### Bug Fixes

- Add username to webhook JSON in discord handlers

## [1.0.1] - 2023-10-18

### Miscellaneous Tasks

- Simplify environment conditional in `deploy.yml`

## [1.0.0] - 2023-10-18

### Documentation

- Update README with better project information
- Create `CONTRIBUTING.md` for project guidelines

### Miscellaneous Tasks

- Add configurations for multiple environments in `wrangler.toml`
- Improve dev vars example for better readability
- Add wrangler vars files to `.gitignore`

## [1.0.0-beta.0] - 2023-10-18

### Refactor

- Replace `log` crates with built-in `worker` console log
- Update logging level from Debug to Trace

### Revert

- Replace `log` crates with built-in `worker` console log

## [1.0.0-alpha.57] - 2023-10-16

### Bug Fixes

- Corrected API endpoint url in prop lot farcaster
- Update cache key in lil nouns farcaster
- Add check for empty cast hash before making request

### Refactor

- Improve JSON construction in FarCaster handlers

## [1.0.0-alpha.56] - 2023-10-16

### Miscellaneous Tasks

- Update deploy environment setup

## [1.0.0-alpha.55] - 2023-10-16

### Features

- Post farcaster votes and comments as thread replay casts

### Miscellaneous Tasks

- Update GitHub deploy workflow for environment handling

## [1.0.0-alpha.54] - 2023-10-11

### Miscellaneous Tasks

- Add more directories to `.dockerignore`
- Update Dockerfile to simplify copying commands

## [1.0.0-alpha.53] - 2023-10-11

### Revert

- Update `build` command in `wrangler.toml`

## [1.0.0-alpha.52] - 2023-10-11

### Documentation

- Add development guide to README

### Miscellaneous Tasks

- Specify Rust version in `Cargo.toml`
- Update `package.json` with `engine` versions
- Add Docker setup for Rust project
- Update `build` command in `wrangler.toml`
- Update `dev` script in `package.json`
- Add `.dev.vars.example` file and update `Dockerfile`

### Revert

- Update `dev` script in `package.json`

## [1.0.0-alpha.51] - 2023-10-10

### Bug Fixes

- Update lil nouns voting descriptions in discord handler

### Refactor

- Add Link utility to lil nouns FarcasterHandler

## [1.0.0-alpha.50] - 2023-10-09

### Refactor

- Change Vote id type to String on lil nouns module

### Revert

- Disable lil nouns in development environment

## [1.0.0-alpha.49] - 2023-10-09

### Miscellaneous Tasks

- Disable lil nouns in development environment

## [1.0.0-alpha.48] - 2023-10-09

### Refactor

- Update logging for error handling
- Combine ProposalQuery and VoteQuery into single struct for lil nouns

### Documentation

- Update social media follow badge in README

### Miscellaneous Tasks

- Combine ProposalQuery and VoteQuery for lil nouns

## [1.0.0-alpha.47] - 2023-10-08

### Bug Fixes

- Update prop house vote message to include proposal title

## [1.0.0-alpha.46] - 2023-10-07

### Miscellaneous Tasks

- Add Lil Nouns deploy secrets to GitHub workflow

## [1.0.0-alpha.45] - 2023-10-07

### Features

- Add LilNouns module with GraphQL fetcher and handlers

### Refactor

- Update code structure for clarity and consistency
- Update Proposal and Vote structures to add detail on Lil Nouns
- Add new functions to handle Discord messages for Lil Nouns
- Implement handling for new Lil Nouns proposals and votes
- Add HTTP request handling function in Lil Nouns farcaster
- Change data type of Lil Nouns proposal id in Vote struct to usize

### Miscellaneous Tasks

- Add new Lil Nouns GraphQL schema
- Add Lil Nouns GraphQL queries for proposals and votes
- Add configuration for Lil Nouns in `wrangler.toml`

## [1.0.0-alpha.44] - 2023-10-05

### Refactor

- Add timestamp to link generation

## [1.0.0-alpha.43] - 2023-10-05

### Miscellaneous Tasks

- Add pnpm package manager installation step to deploy workflow

## [1.0.0-alpha.42] - 2023-10-05

### Features

- Add link generation utility

### Refactor

- Move panic hook configuration to utils module
- Improve error handling in link generator
- Add `Link` utils to `FarcasterHandler` for Prop Lot URL generation.
- Update discord and farcaster event descriptions for prop lot
- Update vote and proposal message format and add Link utility support

### Miscellaneous Tasks

- Add `.editorconfig` for coding consistency

## [1.0.0-alpha.41] - 2023-09-29

### Refactor

- Update warp cast channel settings

## [1.0.0-alpha.40] - 2023-09-28

### Refactor

- Update `get_wallet_handle` function for clarity

## [1.0.0-alpha.39] - 2023-09-28

### Refactor

- Update abstain vote terminology in farcaster
- Update FarcasterHandler to accept dynamic channel key

## [1.0.0-alpha.38] - 2023-09-28

### Refactor

- Update vote description formatting in farcaster

## [1.0.0-alpha.37] - 2023-09-28

### Refactor

- Truncate and append ellipsis to lengthy comments on prop lot farcaster
- Update vote description format in prop lot farcaster
- Move utils into mod directory for improved structure
- Move `get_domain_name` function to new module
- Update ENS Utils and add `get_domain_field` function
- Update provider creation logic for ENS utility
- Replace wallet address fetching method

## [1.0.0-alpha.36] - 2023-09-28

### Refactor

- Change prop lot idea title to uppercase in comment description of farcaster

## [1.0.0-alpha.35] - 2023-09-28

### Refactor

- Update farcaster notification message for prop lot
- Update comment description format for prop lot farcaster
- Improve string truncation in farcaster of prop lot

## [1.0.0-alpha.34] - 2023-09-28

### Refactor

- Update vote choice categorization in discord and farcaster handlers of meta gov

## [1.0.0-alpha.33] - 2023-09-28

### Miscellaneous Tasks

- Add MetaGov webhook and token to deploy workflow

## [1.0.0-alpha.32] - 2023-09-28

### Refactor

- Rename `from` methods to `new_from_env`
- Create GraphQL queries for snapshot data
- Add MetaGov module for fetching and handling proposals and votes
- Add proposal and vote handling in DiscordHandler of meta gov
- Updated visibility and added proposal reference to Vote in `meta_gov` module
- Integrate MetaGov into startup process
- Implement handling for new proposals and votes
- Improve module initialization and clean up imports
- Change data type of choice and remove metadata for meta gov fetcher
- Add function to extract proposal information on meta gov handler
- Remove duplicate code in meta gov discord handler
- Add environment based handler initializations
- Add a function to extract proposal information on meta gov farcaster

### Miscellaneous Tasks

- Add GraphQL schema for data queries in Snapshot
- Add meta gov dev environment variables
- Add new configurable variables to dev environment
- Enable Farcaster, Prop House, and Prop Lot functionalities

## [1.0.0-alpha.31] - 2023-09-27

### Refactor

- Enable channel key for Farcaster handlers

## [1.0.0-alpha.30] - 2023-09-26

### Refactor

- Replace incorrect quotation marks in string formatting

### Miscellaneous Tasks

- Add `rustfmt.toml` configuration file

### Styling

- Refactor code to improve readability

## [1.0.0-alpha.29] - 2023-09-26

### Refactor

- Move handlers into respective module directories
- Move Discord handlers into separate modules
- Update notification string formatting in discord handlers
- Add Farcaster handler in prop lot module
- Add Handler trait to `prop_lot` module
- Update Discord and Farcaster handler to use async traits
- Integrate Handler trait with PropLot struct
- Optimize imports and modify fetcher in `prop_lot` module
- Update visibility of Idea, Vote, Comment struct and their methods
- Improve format and remove unused variables in FarcasterHandler
- Update `async_trait` definition for Handlers
- Improve code modularity and caching utility
- Implement Handler trait for DiscordHandler
- Add FarcasterHandler to manage incoming requests in Prop House
- Update farcaster for better readability
- Update struct and function visibility in fetcher
- Update handlers to support multiple types
- Update PropLot to support multiple handlers

### Miscellaneous Tasks

- Add Warp Cast tokens to deploy workflow

## [1.0.0-alpha.28] - 2023-09-26

### Miscellaneous Tasks

- Update `Cargo.toml` with author details
- Update `kv_namespaces` and var binding in `wrangler` config

## [1.0.0-alpha.27] - 2023-09-25

### Refactor

- Remove `fundingAmount` field from prop house query

## [1.0.0-alpha.26] - 2023-09-25

### Miscellaneous Tasks

- Update community ID value data type in `wrangler.toml`

## [1.0.0-alpha.25] - 2023-09-25

### Refactor

- Switch fetching of Discord Webhooks URLs to secrets

### Miscellaneous Tasks

- Update `wrangler.toml` configuration for `dev` environment
- Add `skip_serializing_none` option to queries in prop house fetcher

## [1.0.0-alpha.24] - 2023-09-25

### Refactor

- Remove redundant main function in lib

### Miscellaneous Tasks

- Add `unbound` usage model to `wrangler` config
- Add environment variables to `wrangler.toml`
- Add Ethereum Mainnet RPC URL to `wrangler.toml`
- Simplify workflow by removing unused secrets

## [1.0.0-alpha.23] - 2023-09-25

### Miscellaneous Tasks

- Update ethers to disable default features
- Enable `logpush`, disable `workers_dev` in config for `wrangler`

## [1.0.0-alpha.22] - 2023-09-24

### Refactor

- Update error handling in webhook execution
- Rename function and add address helper functions
- Update handlers to improve user-friendly naming

## [1.0.0-alpha.21] - 2023-09-24

### Refactor

- Add utility for resolving ENS from Ethereum addresses
- Add ENS support for handlers in prop lot and prop house

## [1.0.0-alpha.20] - 2023-09-23

### Miscellaneous Tasks

- Update deployment configuration in GitHub Actions

## [1.0.0-alpha.19] - 2023-09-23

### Refactor

- Improve cache access efficiency for setup dates
- Update cache retrieval to avoid panic
- Improve error handling in PropLot and PropHouse initialization
- Update cron function for better error handling
- Update codebase to use PropHouse and PropLot structs
- Add cache checking before fetching data
- Remove unnecessary logging in handlers and cache
- Update cache `has` method to use store list
- Update debug statement in start function
- Simplify logging in fetcher modules
- Revise graphql queries and fetcher for prop house

### Miscellaneous Tasks

- Add Cloudflare API token to GitHub workflows
- Update `deploy` script in `package.json`

## [1.0.0-alpha.18] - 2023-09-23

### Refactor

- Add Clone trait to struct in fetchers for performance optimization
- Improve caching for auctions, proposals, votes, ideas, and comments

### Miscellaneous Tasks

- Remove unused dependencies from `Cargo.toml`
- Add metadata to `Cargo.toml`

### Revert

- Update deployment command in GitHub Actions

## [1.0.0-alpha.17] - 2023-09-22

### Miscellaneous Tasks

- Update deployment command in GitHub Actions

## [1.0.0-alpha.16] - 2023-09-22

### Refactor

- Remove HTML parsing from property description

### Miscellaneous Tasks

- Update wrangler dev command in `package.json`

## [1.0.0-alpha.15] - 2023-09-21

### Refactor

- Update `kv_namespaces` configuration in `wrangler.toml`

## [1.0.0-alpha.14] - 2023-09-21

### Refactor

- Improve error handling and logging in fetchers
- Add debug logging to cache operations

### Miscellaneous Tasks

- Update `build` command in `wrangler.toml`
- Add new kv_namespace to wrangler.toml

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
