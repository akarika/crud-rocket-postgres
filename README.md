# Prerequuuuuuuuuuuuuuuuuuuuuuisites

- [Docker](https://docker.com)
- [Docker Compose](https://docs.docker.com/compose)

# Quickstart

```bash
mkdir -p data/db data/reverse-proxy
touch data/reverse-proxy/acme.json
chmod 600 data/reverse-proxy/acme.json
docker-compose up -d --remove-orphans
```
