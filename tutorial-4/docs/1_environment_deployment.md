# Setup Development Envs for Graph Node 

GraphNode stores its internal graph abstraction as schemas within database tables. Therefore, to set up a runnable environment for GraphNode, it is essential to configure database connection information in the GraphNode Docker configuration. 

GraphNode provides two primary methods for setting up a local environment: 

1. Standalone GraphNode Setup 
This approach involves setting only the GraphNode environment by specifying the database connections details directly in its Dockerfile. This is suitable for cases where an external database is already setup and accessbile. 

2. Integrated Setup with GraphNode, IPFS, and Database 

This method configures an environment where GraphNode operates alongside IPFS and a database in an integrated manner. After setting up the necessary Docker configurations, you can start the environment by running: 

```shell 
docker compose up -d docker-compose.yml 
```


## References
* [GraphNode Docker Images]()
* [GraphNode Metadata(DB Table) Schema Introduction]()