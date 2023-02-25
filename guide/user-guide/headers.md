## Send request with headers
Simply to add a header to request, you need to pass a dict with headers into `reqsnaked.Request` construction
```python
request = reqsnaked.Request(
    "GET", "https://httpbin.org/anything",
    headers={"X-Foo": "bar"}
)
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client()
        request = reqsnaked.Request(
            "GET", "https://httpbin.org/anything",
            headers={"X-Foo": "bar"}
        )
        response = await client.send(request)
        content = await response.json()
        # httpbin returns headers we passed in the response
        content.show("headers")


    asyncio.run(main())
    ```
***
Whether all your requests use the same headers, they could be provided using preset in `reqsnaked.Client`
```python
client = reqsnaked.Client(headers={"X-Foo": "Bar"})
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client(headers={"X-Foo": "Bar"})
        request = reqsnaked.Request("GET", "https://httpbin.org/anything")
        response = await client.send(request)
        content = await response.json()
        # httpbin returns headers we passed in the response
        content.show("headers")


    asyncio.run(main())
    ```
***
You can also specify `User-Agent` directly using `user_agent=` parameter
```python
client = reqsnaked.Client(user_agent={"User-Agent": "Reqsnaked/0.1"})
```


To setup same headers for any request, use `headers=` parameter. All headers you pased as default could be overwritten in a certain request.

```python
client = reqsnaked.Client(headers={"X-Foo": "Bar"})
request = reqsnaked.Request("GET", "https://httpbin.org/anything")
response = await client.send(request)
content = await response.json()
# httpbin returns headers we passed in the response
content.show("headers")
```
```json
{
    "Accept": "*/*",
    "Accept-Encoding": "gzip, br",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-63f9ddb5-2618225027ab4df5375f7843",
    "X-Foo": "Bar"
}
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client(headers={"X-Foo": "Bar"})
        request = reqsnaked.Request("GET", "https://httpbin.org/anything")
        response = await client.send(request)
        content = await response.json()
        # httpbin returns headers we passed in the response
        content.show("headers")


    asyncio.run(main())
    ```
You can also specify `User-Agent` directly with `user_agent=`
```python
client = reqsnaked.Client(user_agent="Reqsnaked/0.1")
```


## Read headers from response
To access response headers use `reqsnaked.Response.headers` property where values are always `#!python bytes()` instance:
```python
response = await client.send(request)
date = response.headers["date"]
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client(headers={"X-Foo": "Bar"})
        request = reqsnaked.Request("GET", "https://httpbin.org/anything")
        response = await client.send(request)
        date = response.headers["date"]


    asyncio.run(main())
    ```

Headers are `reqsnaked.HeaderMap` object and can be accessed directly using `__getitem__` or through `.to_dict()` conversion:

```python
response = await client.send(request)
assert response.headers["date"] == response.headers.to_dict()["date"]
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client(headers={"X-Foo": "Bar"})
        request = reqsnaked.Request("GET", "https://httpbin.org/anything")
        response = await client.send(request)
        assert response.headers["date"] == response.headers.to_dict()["date"]


    asyncio.run(main())
    ```

!!! Note
    `__getitem__` approach supports case-insensetive access to headers. When `HeaderMap` converted to dict, all header names will be lowercased
