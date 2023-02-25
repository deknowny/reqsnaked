# Send request with query string
Simply to add query params to request, you need to pass a dict with params into `reqsnaked.Request` construction.

```python
request = reqsnaked.Request(
    "GET", "https://httpbin.org/anything",
    query={"foo": "bar"}
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
            query={"foo": "bar"}
        )
        response = await client.send(request)
        content = await response.json()
        # httpbin returns query string we passed in the response
        content.show("args")


    asyncio.run(main())
    ```
    ```json
    {"foo": "bar"}
    ```


You can also combine query string from params and hardcoded (params will override hardcoded)

```python hl_lines="2"
request = reqsnaked.Request(
    "GET", "https://httpbin.org/anything?fizz=bazz",
    query={"foo": "bar"}
)
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client()
        request = reqsnaked.Request(
            "GET", "https://httpbin.org/anything?fizz=bazz",
            query={"foo": "bar"}
        )
        response = await client.send(request)
        content = await response.json()
        # httpbin returns query string we passed in the response
        content.show("args")


    asyncio.run(main())
    ```
    ```json
    {
        "fizz": "bazz",
        "foo": "bar"
    }
    ```

***
It supports a few standart python types so params like this:
```python
query={
    "array": ["bar", -123, False],
    "nothing": None,
    "float": 6.332,
    "boolean": True
}
```

Will produce such query string
```url
nothing=null&boolean=1&array=bar&array=-123&array=0&float=6.332
```
