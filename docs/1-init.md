# 准备工作

- 创建后台库

```shell
podman run -dt --name postgres-240221 -e POSTGRES_PASSWORD=1234 -v "/home/chin/files-ext/others/postgres:/var/lib/postgresql/data:Z" -p 5432:5432 postgres

CREATE DATABASE nodetree;
create user nodetree with encrypted password 'nodetree';

ALTER DATABASE nodetree OWNER TO nodetree;
GRANT ALL PRIVILEGES ON DATABASE nodetree TO nodetree;

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO nodetree;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO nodetree;
```
