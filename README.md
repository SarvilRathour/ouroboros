# ouroboros
** Docker Setup **
```
docker run --name ouroboros \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_USER=sarvil \
    -e POSTGRES_DB=cyclic \
    -p 5432:5432 \
    -d postgres
```
> If persisting data is needed use: -v pgdata:/var/lib/postgresql/data \

```
DATABASE_URL=postgres://sarvil:password@localhost:5432/cyclic
```
