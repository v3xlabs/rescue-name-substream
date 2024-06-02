# 🛟 [rescue.name](https://rescue.name) Substream

An decentralized vault-based ENS renewal manager.

> 👷🏽‍♀️ Hackathon Project
>
> This project was created at [ETHPrague 2024](https://ethprague.com)

This repository contains the substream belonging to [v3xlabs/rescue-name](https://github.com/v3xlabs/rescue-name). Please see this repository for more information.

## Deployment

Hosted at [StreamFast](https://srv.streamingfast.io/9e804b35/graphiql).

```
make build
substreams pack substreams.sql.yaml
substreams alpha service deploy rescue-name-v0.1.0.spkg -e https://deploy.streamingfast.io --prod
```

## Copyright

All code is licensed under [GPL-3.0](./LICENSE).
