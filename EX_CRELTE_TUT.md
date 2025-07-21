
```
```


```bash
cd craft
docker buildx build --platform linux/amd64 --push -t "registry.b2.goodserver.ch/crelte-tut/craft:latest" .
```

```bash
cd svelte
docker buildx build --platform linux/amd64 --push -t "registry.b2.goodserver.ch/crelte-tut/svelte:latest" .
```

```compose.yml
services:
  craft:
    image: registry.b2.goodserver.ch/crelte-tut/craft
    restart: on-failure
    # ports:
    #   - "127.0.0.1:3615:80"
    extra_hosts:
      - "host.docker.internal:host-gateway"
    volumes:
      # 3000
      - "./.env:/app/.env:ro"
      - ./storage/logs:/app/storage/logs
      - ./storage/backups:/app/storage/backups
      - ./web/assets:/app/web/assets
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.crelte-tut-craft.rule=Host(`staging.crelte-tut.b2.goodserver.ch`)"
      - "traefik.http.routers.crelte-tut-craft.entrypoints=websecure"
      - "traefik.http.routers.crelte-tut-craft.tls.certresolver=letsencrypt"
      - "traefik.http.services.crelte-tut-craft.loadbalancer.server.port=80"
    networks:
      - traefik

  svelte:
    image: registry.b2.goodserver.ch/crelte-tut/svelte
    restart: on-failure
    # ports:
    #   - "127.0.0.1:3616:8080"
    extra_hosts:
      - "host.docker.internal:host-gateway"
    volumes:
      - "./.env:/craft/.env:ro"
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.crelte-tut-svelte.rule=Host(`crelte-tut.b2.goodserver.ch`)"
      - "traefik.http.routers.crelte-tut-svelte.entrypoints=websecure"
      - "traefik.http.routers.crelte-tut-svelte.tls.certresolver=letsencrypt"
      - "traefik.http.services.crelte-tut-svelte.loadbalancer.server.port=8080"
    networks:
      - traefik

  mysql:
    image: mysql:8.0
    # ports:
    #   - "127.0.0.1:3307:3306"
    volumes:
      # 999
      - ./db:/var/lib/mysql
    environment:
      MYSQL_DATABASE: crelte-tut
      MYSQL_USER: crelte-tut
      MYSQL_PASSWORD: SomePassword123!
      MYSQL_ROOT_PASSWORD: SomePassword123!
    networks:
      - traefik

networks:
  traefik:
    external: true
```

The create the `.env` file.

Then run `docker compose up -d --pull always`.

We can setup a ci:
```docker-build.yml
name: Build and Push Docker Images

on:
  push:
    branches:
      - main    # or whichever branch you want to trigger on
  workflow_dispatch:

jobs:
  setup-docker-buildx:
    runs-on: ubuntu-latest
    outputs:
      builder: ${{ steps.buildx.outputs.name }}

    steps:
      - name: Get latest Git commit (for tags or labels)
        id: git-info
        run: echo "::set-output name=sha::$(git rev-parse --short HEAD)"
      - name: Set up Docker Buildx
        id: buildx
        uses: docker/setup-buildx-action@v3
        with:
          install: true
      - name: Log in to registry.b2.goodserver.ch
        uses: docker/login-action@v2
        with:
          registry: registry.b2.goodserver.ch
          username: ${{ secrets.REGISTRY_USERNAME }}
          password: ${{ secrets.REGISTRY_PASSWORD }}
      - name: Create BuildKit builder
        if: steps.buildx.outputs.name == ''
        run: |
          docker buildx create --name mybuilder --use
      - name: Inspect builder
        run: docker buildx inspect --bootstrap

  build-and-push:
    needs: setup-docker-buildx
    runs-on: ubuntu-latest
    environment: production

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      # --- Build & push `craft` ---
      - name: Build and push craft image
        uses: docker/build-push-action@v4
        with:
          context: ./craft
          platforms: linux/amd64
          push: true
          tags: |
            registry.b2.goodserver.ch/crelte-tut/craft:latest
          cache-from: type=registry,ref=registry.b2.goodserver.ch/crelte-tut/craft:latest
          cache-to: type=registry,ref=registry.b2.goodserver.ch/crelte-tut/craft:cache,mode=max

      # --- Build & push `svelte` ---
      - name: Build and push svelte image
        uses: docker/build-push-action@v4
        with:
          context: ./svelte
          platforms: linux/amd64
          push: true
          tags: |
            registry.b2.goodserver.ch/crelte-tut/svelte:latest
          cache-from: type=registry,ref=registry.b2.goodserver.ch/crelte-tut/svelte:latest
          cache-to: type=registry,ref=registry.b2.goodserver.ch/crelte-tut/svelte:cache,mode=max
```
