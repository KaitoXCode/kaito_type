# kaito_type

## devops

### todos

(1) dockerise app <br />

(2) (for prod only) persist docker postgres db data <br />

(3) git actions <br />

(4) pipelines <br />

## frontend

### done

(v1)

(a) add simple html home/root and login pages. <br />

(b) incorporate working (json /api/v1) authentication with demo user <br />

(v2)

(a) move css to styles.css in assets <br />

(b) border styles for divs and buttons + general styles across client <br />

(c) add favicon <br />

(d) replace script html component with htmx dynamic component (/api/v2) <br />

### todos

(v3)

(a) incorporate logoff and create user for client (/api/v1) <br />

(e) use tailwind for css (add
`<script src="https://cdn.tailwindcss.com"></script>` to the head)<br />

(v4)

(c) convert login page to dynamic aside component (/api/v2) <br />

## backend

### done

(v1)

(a) (simple) rust/axum backend <br />

(b) (simple) context:ctx <br />

(c) (simple) logging <br />

(e) (fake) model controller, script models <br />

(f) (simple) testing: quick_dev testing <br />

(g) (simple) routes: login (fake auth) and script <br />

(v2)

(a) (complex) rust/axum backend <br />

(b) configs, better logging, modular errors, api context <br />

(c) docker db (w/create;recreate;seeding), db models <br />

(d) (real) auth, cookies, encryption <br />

(e) (v1 endpoints) RPC API <br />

(f) response mapper <br />

(g) some unit tests <br />

(v3)

(a) serve assets directory for frontend <br />

(b) merge html pages to the api using askama templates (create web::html mod)
<br />

(c) nest dynamic htmx components to the api using askama templates (also in
web::html mod) <br />

(d) nest all json transactions with /api/v1 and add/nest html transactions with
/api/v2 <br />

### todos

(d) add json logoff and create user (/api/v1) <br />

(e) working no-auth dynamic content (htmx) call for script getter route /api/v2
<br />

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
