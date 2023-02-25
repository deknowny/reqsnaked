## Send a request
The first step to make any HTTP request is create `reqsnaked.Client` instance. It introduces an asynchronous method to send a `reqsnaked.Request` and enables some customization that shares across all requests. It keeps connection to hosts alive, so it's useful to create only one client.


Client could be created with or without a running event loop instantly. As it does all asynchronoues HTTP libraries, client should be closed in async style after usage, but it's not required for `reqsnaked` because internally it uses RAII approach and connection will be closed automatically.

```python title="A simple GET request"
import asyncio

import reqsnaked


# Note that you should not close the connection
# as it happends using RAII approach
async def main():
    client  = reqsnaked.Client()
    request = reqsnaked.Request("GET", "https://httpbin.org/anything")
    response = await client.send(request)
    print(response.status.value)


asyncio.run(main())
```
```
200
```

## Process the response
After recieving the response, body could be accessed in different ways, for example, as JSON. The client will not await the whole response to be written, so at this step only status code, HTTP version and headers can be access before the body actually will be read, that's why it's neccessary use another one `#!python await`.

```python title="Get response body as JSON" hl_lines="10"
client  = reqsnaked.Client()
request = reqsnaked.Request(
    "GET", "https://httpbin.org/anything",
    query={"foo": "bar"}  # (1)
)
# Await first part of the response
response = await client.send(request)

# Await body and try decode it as JSON
body = await response.json()
body.show("args") # (2)
```

1. Query string are arguments passed after URL's path so actually URL transforms to this:
```url
https://httpbin.org/anything?foo=bar
```
You can read more about how use query string in `reqsnaked` [there](./query-string.md).
2. Httpbin's `/anything` endpoint returns everything we passed, so `#!python .show("args")` used to print `#!python ["args"]` from the response body.

```json
{"foo": "bar"}
```

??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client  = reqsnaked.Client()
        request = reqsnaked.Request(
            "GET", "https://httpbin.org/anything",
            query={"foo": "bar"}  # (1)
        )
        # Await first part of the response
        response = await client.send(request)

        # Await body and try decode it as JSON
        body = await response.json()
        body.show("args")  # (2)


    asyncio.run(main())
    ```

## Read more
Now you can read more about `reqsnaked.Client`, `reqsnaked.Request` and `reqsnaked.Response` in other topics.
