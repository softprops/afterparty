# afterparty

[![Build Status](https://travis-ci.org/softprops/afterparty.svg?branch=master)](https://travis-ci.org/softprops/afterparty)

> what happens on github...

## docs

Find them [here](http://softprops.github.io/afterparty)

# usage

```shell
$ cat afterparty.json
{
  "hooks": [{
    "cmd": "echo \"hit with $GH_EVENT_NAME : $GH_EVENT_PAYLOAD\"",
    "events": ["push"]
  }]
}
$ afterparty -c afterparty.json
```
