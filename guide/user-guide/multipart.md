To send `multipart/form-data` request, pass an object to `#!python multipart=` param

```python title="Send two files"
request = reqsnaked.Request(
    "POST", "https://httpbin.org/anything",
    multipart=reqsnaked.Multipart(
        reqsnaked.Part(
            "foo", b"000"
            filename="foo.txt",
            mime="text/plain"
        ),
        reqsnaked.Part(
            "bar", b"111",
            filename="bar.txt",
            mime="text/plain"
        )
    ),
)
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked

    async def main():
        client = reqsnaked.Client()
        request = reqsnaked.Request(
            "POST", "https://httpbin.org/anything",
            multipart=reqsnaked.Multipart(
                reqsnaked.Part("foo", b"000", filename="foo.txt",
                                mime="text/plain"),
                reqsnaked.Part("bar", b"111", filename="bar.txt",
                            mime="text/plain")
            ),
        )
        response = await client.send(request)
        content = await response.json()
        # httpbin returns query string we passed in the response
        content.show()

    asyncio.run(main())
    ```
    ```json
    {
        "bar": "111",
        "foo": "000"
    }
    ```


!!! TODO
    Currently, only raw bytes and strings are supported for content, `io` objects are coming soon
