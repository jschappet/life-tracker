#!/bin/zsh 

docker build -t life-tracker .  

docker create --name life-tracker-container life-tracker 

docker cp life-tracker-container:/usr/src/app/life-tracker ./life-tracker

docker rm life-tracker-container

ssh www 'ps aux | grep life-tracker | grep -v grep | awk "{print \$2}" | xargs kill -9'

# Sync the migrations, templates, static, and life-tracker files to the server
rsync -a migrations/ www:git/life-tracker/migrations
rsync -a templates/ www:git/life-tracker/templates
rsync -a static/ www:git/life-tracker/static
rsync -a life-tracker.sh life-tracker www:git/life-tracker

# Restart the server without waiting for the files to sync
ssh www -x 'cd git/life-tracker && nohup ./life-tracker.sh &'
