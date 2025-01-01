# DemocraSee

## Development
### Running API Server
- It runs =SERVICE= crate.
  - Default =SERVICE= is =main-ui=.

``` bash
export SERVICE=main-api
make run
```

### Running Web UI
- It will interact with API server in `dev` environment.
  - If you want to change it, set `MAIN_API_ENDPOINT` environment.
- Before running UI, set up firebase configuration

``` bash
export SERVICE=main-ui
export MAIN_API_ENDPOINT=http://localhost:3000

export FIREBASE_API_KEY=""
export FIREBASE_AUTH_DOMAIN=""
export FIREBASE_PROJECT_ID=""
export FIREBASE_STORAGE_BUCKET=""
export FIREBASE_MESSAGING_SENDER_ID=""
export FIREBASE_APP_ID=""
export FIREBASE_MEASUREMENT_ID=""

make run
```

