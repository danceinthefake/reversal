# Reversal 

Flask application that do reversal text in the path and show them in page

## Starting server

### Local
1. Export database variable
```
DATABASE_URL="postgresql://<database_username>:<database_password>@<database_host>:<database_port>/<database_name>"
```
2. Do database init before first access to application
```
flask db init
```
3. Do database migration
```
flask db migrate
flask db upgrade
```
do this again after you change schema of database

### Docker Compose for development only
```
docker-compose -f compose.yml  up
#First time only
docker-compose exec app flask db init
# First time and each time change schema in database
docker-compose exec app flask db migrate
docker-compose exec app flask db upgrade
```

## Access
Access via port 5000
```
curl http://localhost:5000
```
To use reversal functionality, just add path
```
curl http://localhost:5000/wearenotalone
```
and your path become reversal in page
```
enolatoneraew
```