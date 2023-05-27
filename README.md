## Caution
This repo is in progress.

↓ Next actions.
- Cloud Run integration
- Cloud Storage integration


## Motivation
I created a repository to maintain backups of Heroku's PostgreSQL for more than 30 days. In order to keep backups for more than 30 days, it is necessary to subscribe to a premium plan or higher, which costs over $200 per month.

By using Cloud Run and Cloud Storage, the goal is to save the daily backup dumps and thus keep costs down.


## How to use
```
cp .env.dist .env
```
Set your env vals.

```
docker-compose build
docker-compose up
```
