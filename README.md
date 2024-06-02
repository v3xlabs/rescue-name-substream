# Rescue Name Substream

Hosted at [StreamFast](https://srv.streamingfast.io/9e804b35/graphiql).

## Steps to setup

```
make build
substreams pack substreams.sql.yaml
substreams alpha service deploy rescue-name-v0.1.0.spkg -e https://deploy.streamingfast.io --prod
```

