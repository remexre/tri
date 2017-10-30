# tri

A simple task bot for Slack.

Depends on the environment variables `DATABASE_URL` and `SLACK_API_TOKEN`.
These environment variables may be set in a `.env` file.

## Usage

```
$ docker build -t remexre/tri .
$ docker run --rm -it --env SLACK_API_TOKEN=your-slack-api-token remexre/tri
```
