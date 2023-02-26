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
{'url': 'https://httpbin.org/anything', 'method': 'POST', 'json': None, 'files': {'foo': '01010101'}, 'headers': {'Content-Type': 'multipart/form-data; boundary=1d0854194a7a7096-e4818bfc797d4d8f-71250e8530573ad3-9f55410350495f98', 'Accept-Encoding': 'gzip, br', 'Host': 'httpbin.org', 'X-Amzn-Trace-Id': 'Root=1-63f93eb3-70555b2139c89c5a29a49d37', 'Accept': '*/*', 'User-Agent': 'Reqsnaked/1.0', 'X-Foo': 'bar', 'Content-Length': '246', 'X-Bar': 'foo'}, 'origin': '185.97.201.3', 'form': {}, 'data': '', 'args': {}}
```

## Installlation
Currently the library is not published to PyPI, so the only way to install it is from GitHub:
```bash
python -m pip install -U https://github.com/deknowny/reqsnaked/archive/main.zip
```


## Features list
The library is in developing stage and these features are coming soon.
- [X] Redirect policies in client
- [X] Form requests
- [X] Multipart requests
- [X] Raw body reader
- [X] Chunks body reader
- [X] Cast `urllib.parse.ParseResult` for request building
- [X] Cast `http` enums for request building
- [ ] Proxy policy (random and cycled)
- [ ] Rate limiter
- [ ] Cookies
- [X] Own exceptions
- [ ] Pyi stubs
