# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`lecto_client` is a private Rust async HTTP client library for the Lecto debt management API. It provides type-safe interfaces for managing debtors, debts, debt statuses, and reminder notifications.

## Common Commands

```bash
# Build
cargo build

# Run all tests
cargo test --all-features

# Run a single test
cargo test <test_name>

# Format check (CI enforces this)
cargo fmt --all -- --check

# Lint (CI enforces this)
cargo clippy --all-features
```

CI tests against Rust stable and 1.85.0.

## Architecture

### Module Layout

- **`client.rs`** — Core `Client` struct with async HTTP methods (`post_debtor`, `post_debt`, `patch_debt_statuses`, `get_reminds`). Includes retry logic with exponential backoff and `LectoError` enum for structured error handling.
- **`debtor.rs`** — Debtor data models. Uses an internal `DebtorRawRequest` type to handle the API's `kyc_done` as integer (0/1) while exposing it as `bool` in the public API via `From` conversions.
- **`debt.rs`** — Debt types. Custom fields use `HashMap` but serialize as `BTreeMap` for consistent JSON ordering.
- **`debt_status.rs`** — `DebtStatusVariable` enum and related request/response types.
- **`remind_group/remind.rs`** — Reminder types. Internal `RemindResponse` converts to public `Remind` type.
- **`util.rs`** — URL joining utility.
- **`fixture.rs`** — Test fixtures (compiled only in test cfg).

### Key Patterns

- **Response type conversions**: Internal `*Response` types handle API quirks, then convert to public types via `From` implementations.
- **Retry strategy**: Configurable `max_retry` with 1000ms backoff. Stops early on 200/422/400; retries on network errors or unexpected status codes.
- **Auth**: Bearer token via `api_key` in request headers.
- **Test infrastructure**: Uses `mockito` for HTTP mocking, `rstest` for parameterized tests. Test data in `test-data/`.
