## Database Migrations

### Prerequisites

You need `sqlx-cli` to be installed. To install it, use:

```
cargo install --version=0.7.4 sqlx-cli --no-default-features --features native-tls,postgres
```

On an Ubuntu based Linux distro, you need to have `libssl-dev` package installed,
so that `sqlx-cli` can be compiled and installed. Install it using:

```
sudo apt-get install libssl-dev
```

<br/>

### Init Database

Use `./db_init.sh` to create and initialize (populating it with all the changes that exist) the database as a Docker container.

<br/>

### Apply Changes

Newer database changes introduced during development are applied as follows:

1. Create the change using `./db_add_change.sh {change-name}`.<br/>
   Ex: `./db_add_change.sh create_table_user_accounts`

2. Populate the generated file.<br/>

3. Apply the change using `./db_apply_changes.sh`.
