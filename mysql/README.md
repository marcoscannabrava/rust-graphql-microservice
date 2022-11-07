# Dev Database Helper

Some helpful commands:
```sh
docker exec -it mysql bash               # opens container shell
$containershell> mysql --password=mysql  # connects MySQL CLI to running MySQL instance in the container 
# see MySQL CLI reference for more helpful commands
```

To re-run or change the seed script use MySQL cli or remove mysql-db volume and restart the container.
```sh
docker container rm $(docker container ls -aq -f name=mysql)
docker image rm mysql_mysql-db
```
