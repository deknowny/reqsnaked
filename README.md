# Reqsnaked
Reqsnaked is a blazing fast async/await HTTP client for Python written on Rust using reqwests.

* Works 15% faster at avarage
* RAII approach without context managers
* Memory-efficient lazy JSON parser
* Fully-typed even being written on Rust

***
__Docs__: [https://deknowny.github.io/reqsnaked/devel/](https://deknowny.github.io/reqsnaked/devel/)

## Overview
```python title="Example"
import asyncio
import datetime

import reqsnaked


async def main():
    client = reqsnaked.Client(
        user_agent="Reqsnaked/1.0",
        headers={"X-Foo": "bar"},
        store_cookie=True
    )
    request = reqsnaked.Request(
        "POST", "https://httpbin.org/anything",
        multipart=reqsnaked.Multipart(
            reqsnaked.Part(
                "foo", b"01010101",
                filename="foo.txt",
                mime="text/plain"
            )
        ),
        query={"foo": "bar"},
        headers={"X-Bar": "foo"},
        timeout=datetime.timedelta(seconds=30),
    )
    response = await client.send(request)
    print(response.status)
    data = await response.json()
    data.show()


asyncio.run(main())
```
```
HTTPStatus.OK
```
```json
{
  "args": {
    "foo": "bar"
  },
  "data": "",
  "files": {
    "foo": "01010101"
  },
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Accept-Encoding": "gzip, br",
    "Content-Length": "246",
    "Content-Type": "multipart/form-data; boundary=a59212f1bfcc112f-b3b83c8afd39b140-f302f74df067620a-a8a38a37c3355abe",
    "Host": "httpbin.org",
    "User-Agent": "Reqsnaked/1.0",
    "X-Amzn-Trace-Id": "Root=1-63fcfd9b-412668b5117367524668f43b",
    "X-Bar": "foo",
    "X-Foo": "bar"
  },
  "json": null,
  "method": "POST",
  "origin": "91.223.70.172",
  "url": "https://httpbin.org/anything?foo=bar"
}
```

## Installlation
Currently the library is not published to PyPI, so the only way to install it is from GitHub:
```bash
python -m pip install -U https://github.com/deknowny/reqsnaked/archive/main.zip
```
