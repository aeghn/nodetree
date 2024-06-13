# 准备工作

- 创建后台库

```shell
podman run -dt --name postgres-240221 -e POSTGRES_PASSWORD=1234 -v "/home/chin/files-ext/others/postgres:/var/lib/postgresql/data:Z" -p 5432:5432 postgres

CREATE DATABASE chnots;
create user chnots with encrypted password 'chnots';

ALTER DATABASE chnots OWNER TO chnots;
GRANT ALL PRIVILEGES ON DATABASE chnots TO chnots;

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO chnots;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO chnots;
```
