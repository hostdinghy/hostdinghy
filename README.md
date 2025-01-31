
containers docker, multiple allowed

object storage (for files)

database

router / loadbalancer




we provide:
- container registry
- object storage (might just be folders for the moment)
- postgres as a database

a sort of docker compose file which provides these features

building is done via the ci / cd pipeline
once a projects container get's pushed to our registry we handle the deployment

each project should contain a configuration file


todo kurbenetes

we use hyper, instant-acme, tokio
for docker we could use bollard

sozu or linkerd might be interesting





each instance can be the following things or all of them:
- postgres (database)
- redis (cache)
- object storage


first prototype:
- create volumes
- receive container updates
- let's encrypt
- reverse proxy
