To access raw response, use `#!python .read()` method on response. It returns a special object `reqsnaked.Bytes` and now it could only be converted to python's `#!python bytes()` instance using `#!python .as_bytes()`:

```python title="Read raw body"
client = reqsnaked.Client()
request = reqsnaked.Request(
    "GET", "https://httpbin.org/bytes/10",  # (1)
)
response = await client.send(request)
data = await response.read()
print(data.as_bytes())
```

1. This endpoint returns 10 random bytes

```
b'4NI\xff\x11\x0b\x82E\xb3\xa7'
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client()
        request = reqsnaked.Request(
            "GET", "https://httpbin.org/bytes/10",
        )
        response = await client.send(request)
        data = await response.read()
        print(data.as_bytes())


    asyncio.run(main())
    ```

!!! Failure
    You cannot read from body twice
