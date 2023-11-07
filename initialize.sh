#!/usr/bin/env zsh

# Make sure colima is running
colima status >& /dev/null
if [ $? = 1 ]; then
    echo "starting colima"
    colima start >& /dev/null
else
    echo "colima is running"
fi

# clean up from previousl runs
docker-compose down >& /dev/null
if [ $? = 1 ]; then
    echo '"docker-compose down" threw an error'
    exit 1
fi

docker volume rm tiny_url_db >& /dev/null

docker-compose up -d >& /dev/null
if [ $? = 1 ]; then
    echo '"docker-compose up -d" threw an error'
    exit 1
fi
