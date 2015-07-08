# afterparty

> what happens on github...

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



