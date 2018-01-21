# tri

[![Build Status](https://travis-ci.org/remexre/sparkly-rs.svg?branch=master)](https://travis-ci.org/remexre/sparkly-rs)

A simple task bot for Slack.

Depends on the environment variables `DATABASE_URL` and `SLACK_API_TOKEN`.
These environment variables may be set in a `.env` file.

## Usage

```
$ docker build -t remexre/tri .
$ docker run --rm -it --env SLACK_API_TOKEN=your-slack-api-token remexre/tri
```

## License

Licensed under either of

 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
