Chunks is way to read response body partially without awaiting when the whole body will be sent. It's useful when response is large. Use this pattern to iterate over chunks

```python title="Get image by parts"
client = reqsnaked.Client()
request = reqsnaked.Request(
    "GET", "https://httpbin.org/image/jpeg",  # (1)
)
response = await client.send(request)
stream = response.to_stream()
while chunk := await stream.gnaw():
    print(chunk)
```

1. This endpoint returns an image

```
<reqsnaked.Bytes object at 0x7ff54823ffb0>
<reqsnaked.Bytes object at 0x7ff54823f4f0>
<reqsnaked.Bytes object at 0x7ff54823fcb0>
<reqsnaked.Bytes object at 0x7ff54823ffb0>
<reqsnaked.Bytes object at 0x7ff54823f4f0>
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client()
        request = reqsnaked.Request(
            "GET", "https://httpbin.org/image/jpeg",
        )
        response = await client.send(request)
        stream = response.to_stream()
        while chunk := await stream.gnaw():
            print(chunk)

    asyncio.run(main())
    ```

!!! Failure
    You cannot read from body when creating a streamer
