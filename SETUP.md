# Setup

Everything here should be automated.

## Setup docker

https://soerenmeier.ch/blog/setup-docker

## Define folder
We need to define a folder where everything should be stored.

Modify /etc/environment with the following line:
```
HUUS_DIR="/huus"
```

## Setup traefik

`sudo mkdir ${HUUS_DIR}/traefik`

the traefik folder needs to be owned by root.

```compose.yml
services:
  traefik:
    image: "traefik:v3"
    ports:
      - "80:80"
      - "443:443"
      - "8080:8080"
    networks:
     - traefik
    extra_hosts:
      - "host.docker.internal:host-gateway"
    volumes:
      - "/var/run/docker.sock:/var/run/docker.sock:ro"
      - "./traefik.yml:/etc/traefik/traefik.yml:ro"
      - "./dynamic.yml:/etc/traefik/dynamic.yml:ro"
      - "./letsencrypt:/letsencrypt"
networks:
  traefik:
    external: true
```

```traefik.yml
global:
  checkNewVersion: true
  sendAnonymousUsage: true

log:
  level: DEBUG

entryPoints:
  web:
    address: :80
    http:
      redirections:
        entrypoint:
          to: websecure
          scheme: https
          priority: 10

  websecure:
    address: :443

api:
  dashboard: true

certificatesResolvers:
  letsencrypt:
    acme:
      email: soeren@meierlabs.ch
      storage: /letsencrypt/acme.json
      httpChallenge:
        # used during the challenge
        entryPoint: web

providers:
  # Enable Docker configuration backend
  docker: {}
  file:
    filename: /etc/traefik/dynamic.yml
```

```dynamic.yml
http:
  routers:
    dashboard:
      rule: "Host(`dashboard.b2.goodserver.ch`)"
      service: api@internal
      tls:
        certResolver: letsencrypt
      middlewares:
        - auth

  middlewares:
    auth:
      basicAuth:
        users:
          - "test:$2a$12$b5Od6Dmn1cWAw25kIvrcYuTY67RbF81Dpz5njSBZCtu.aHX/zSeUa"
```

Run `docker network create traefik` to create the network.

Then run `docker compose up -d --pull always`.

## Setup registry

Create folder `sudo mkdir ${HUUS_DIR}/registry`

```compose.yml
services:
  registry:
    image: "registry:2"
    networks:
     - traefik
    environment:
      - SERVICE_FQDN_REGISTRY="https://registry.b2.goodserver.ch"
    volumes:
      - "./registry.password:/auth/registry.password:ro"
      - "./config.yml:/etc/docker/registry/config.yml:ro"
      - "./data:/var/lib/registry"
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.registry.rule=Host(`registry.b2.goodserver.ch`)"
      - "traefik.http.routers.registry.entrypoints=websecure"
      - "traefik.http.routers.registry.tls.certresolver=letsencrypt"
      - "traefik.http.services.registry.loadbalancer.server.port=5000"
networks:
  traefik:
    external: true
```

```config.yml
version: 0.1
log:
  fields:
    service: registry
storage:
  cache:
    blobdescriptor: inmemory
  filesystem:
    rootdirectory: /var/lib/registry
auth:
  htpasswd:
    realm: Registry
    path: /auth/registry.password
http:
  addr: :5000
  headers:
    X-Content-Type-Options: [nosniff]
health:
  storagedriver:
    enabled: true
    interval: 10s
    threshold: 3
```

```registry.password
# This file contains the username and password for the registry.
# The password is hashed using bcrypt.
huus:$2a$12$0BfGj92QSXxzAMJLD3dqIu0HlCof4D7l8fZ86Captj.vxEZ39uMUG
```

Then run `docker compose up -d --pull always`.

Now run `docker login registry.b2.goodserver.ch` on the server to login to the registry.
