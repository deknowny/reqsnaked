# Reqsnaked
Reqsnaked is hand-written bindings for Rust's reqwests library providing features over known client-side HTTP Python libraries like lighting speed and RAII approach even in an async style.

## Overview
```python
import asyncio
import datetime

import reqsnaked



async def main():
    client = reqsnaked.AsyncClient(
        user_agent="Reqsnaked/1.0",
        headers={"X-Foo": "bar"}
    )
    response = await client.request(
        reqsnaked.Request(
            "POST",
            "https://httpbin.org/anything",
            query={"foo": "bar"},
            timeout=datetime.timedelta(seconds=30),
            basic_auth=reqsnaked.BasicAuth("John", "D0eeee")
        )
    )
    print(response.status.code)
    data = await response.json()
    print(data.select())


asyncio.run(main())
```
```
200
{'method': 'POST', 'args': {'foo': 'bar'}, 'form': {}, 'files': {}, 'origin': '185.97.201.3', 'headers': {'X-Foo': 'bar', 'Accept-Encoding': 'gzip, br', 'Authorization': 'Basic Sm9objpEMGVlZWU=', 'User-Agent': 'Reqsnaked/1.0', 'Host': 'httpbin.org', 'X-Amzn-Trace-Id': 'Root=1-63f39e3b-4ca125dc59f80534124e6e62', 'Accept': '*/*'}, 'url': 'https://httpbin.org/anything?foo=bar', 'data': '', 'json': None}

```

# Features list
The library is in developing stage and these features are coming soon.
- [X] Redirect policies in client
- [X] Form requests
- [X] Multipart requests
- [X] Raw body reader
- [X] Chunks body reader
- [X] Cast `urllib.parse.ParseResult` for request building
- [X] Cast `http` enums for request building
- [ ] Rust TLS/OpenSSL selection
- [ ] Proxy policy (random and cycled)
- [ ] Rate limiter
- [X] Own exceptions
- [ ] Pyi stubs
