Docs are very good, just read: https://www.sea-ql.org/SeaORM/docs/introduction/orm/

stuff I'm curious about:

### Workspaces

You might notice this project is multiple crates lol (beginner rust eureka).
I learned that you could do separate workspace structures. It's separates the crates so faster build times I think? Think of it like monorepos.

- Workspaces:
  - migration crate -
  - entity crate - where your 'entities' live (table definitions).
  - app crate - application logic. usually where axum should be.

### Migrations

need to install `cargo install sea-orm-cli`
`sea-orm-cli migrate init` - creates a migration create. (tbh what if we could just use)

### new things I learned.

- Prisma + Seaorm (essentially sqlx still) is possible.
- Prisma + sqlx is good too.

GADAMN seaorm is beautiful because it treats my database as the source of truth.
