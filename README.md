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
$ afterparty  afterparty.json
```

## building

* build a release binary

```bash
$ docker run -it --rm -v $(pwd):/source jimmycuadra/rust:1.2.0
cargo build --release
exit
```
