# kaito_type

## backend

### todos

(1) introduce htmx <br /> (2) dockerise app <br /> (3) (for prod only) persist
docker postgres db data <br /> (4) maybe loosen auth reqs for logged off access

### envs

git ignored /backend/.cargo/config.toml file

SERVICE_DB_URL: db connection url. <br /> SERVICE_PWD_KEY: for pwd encryption.
<br /> SERVICE_TOKEN_KEY: for token generation and validation. <br />
SERVICE_TOKEN_DURATION_SEC: for token lifetime. <br /> SERVICE_WEB_FOLDER:
static html location (always: "web-folder/").

### dev start db

start the postgres db using docker image:

```bash
docker run --rm --name pg -p 5432:5432 \
    -e POSTGRES_PASSWORD=welcome \
    postgres:15
```

access psql terminal for pg:

```bash
docker exec -it -u postgres pg psql
```

connect to db (in psql):

```bash
\c app_db
```

### dev watch commands

watch application level logs:

```bash
cd ~/kaito_type/backend
cargo watch -q -c -w src/ -w .cargo/ -x "run"
```

watch test level logs. examples/quick_dev:

```bash
cd ~/kaito_type/backend
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

unit tests (all):

```bash
cd ~/kaito_type/backend
cargo watch -q -c -x "test -- --nocapture"
```

unit tests (filtered):

```bash
cd ~/kaito_type/backend
cargo watch -q -c -x "test model::script::tests::test_create -- --nocapture"
```
