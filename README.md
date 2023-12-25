# kaito_type

## devops

### todos

(1) dockerise app <br />

(2) (for prod only) persist docker postgres db data <br />

## frontend

### done

(v1) add simple html home/root and login pages. incorporate successful (json)
authentication with demo user <br />

### todos

(v2) replace some html components with htmx dynamic components <br />

## backend

### done

(v1) make a simple rust/axum backend. simple context (ctx), logging, (fake)
auth, script models, model controller, all-in-one error file, login routes,
script routes, quick_dev testing <br />

(v2) make a more complex rust/axum backend. configs, docker db (w/
create;recreate;seeding), db models, better logging, modular errors, api
context, auth, cookies, encryption, RPC API, response mapper, unit tests <br />

### todos

(v3) dynamic content for htmx calls <br />

### envs

git ignored /backend/.cargo/config.toml file

SERVICE_DB_URL: docker postgres db connection url. <br /> SERVICE_PWD_KEY: for
pwd encryption (can gen using examples/gen_key).
<br /> SERVICE_TOKEN_KEY: for token generation and validation (can gen using
examples/gen_key). <br /> SERVICE_TOKEN_DURATION_SEC: for token lifetime in
seconds. <br /> SERVICE_WEB_FOLDER: static html location (always:
"web-folder/").

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
