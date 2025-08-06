## Structure of the compose.yml file

### Image

If you use the registry from HostDinghy the image needs to be as follows:
`registry.domain/appid/service:tag` where the tag is optional.

### Traefik

In most cases there are five labels needed per service, for it
to be accessible on the web:

```yaml
      - "traefik.enable=true"
      - "traefik.http.routers.appid-service.rule=Host(`hostdinghy.com`)"
      - "traefik.http.routers.appid-service.entrypoints=websecure"
      - "traefik.http.routers.appid-service.tls.certresolver=letsencrypt"
      - "traefik.http.services.appid-service.loadbalancer.server.port=80"
```

You might notice `appid-service` this is a requirement for all routers.

Each router must have the appid a minus followed by the service name.


### Example
This is the most simple example:
```yaml
services:
  hello:
    image: nginx:latest
    networks:
      - traefik
    restart: unless-stopped
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.firstapp-hello.rule=Host(`hostdinghy.com`)"
      - "traefik.http.routers.firstapp-hello.entrypoints=websecure"
      - "traefik.http.routers.firstapp-hello.tls.certresolver=letsencrypt"
      - "traefik.http.services.firstapp-hello.loadbalancer.server.port=80"
networks:
  traefik:
    external: true
```
