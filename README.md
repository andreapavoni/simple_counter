# Simple Counter

Simple very app to manage counters.

* [Axum](https://github.com/tokio-rs/axum)
* [Askama](https://github.com/djc/askama) + some [HTMx](https://htmx.org)
* [SeaORM](https://www.sea-ql.org/SeaORM/)
* an experimental library to implement a CQRS/ES pattern: [mini_cqrs](https://github.com/andreapavoni/mini_cqrs).

It is clearly overkill for a simple application like this, indeed I'm using it for practicing and experimenting.

## Usage

* clone this repo: `git clone https://github.com/andreapavoni/simple_counter.git`
* create a SQLite database file (using `touch db.sqlite3` works as well)
* copy `.env.example` to `.env` and edit it accordingly
* compile assets: `cd web/ && pnpm release`
* run: `APP_ENV=prod cargo run --release`
* open: `http://localhost:8000/counters`

## Status

**Work In Progress**

It's a fun experiment/PoC/exercise, but definitely not appropriated for any production use (FWIW). I'm thinking about making it production-approachable to keep as an example but I don't have any clear plans or deadlines.
