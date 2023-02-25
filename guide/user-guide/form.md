# Send `application/x-www-form-urlencoded`
Simply to add form to request, you need to pass a dict with params into `reqsnaked.Request` construction.

```python
request = reqsnaked.Request(
    "POST", "https://httpbin.org/anything",
    form={"foo": "bar"}
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
            form={"foo": "bar"}
        )
        response = await client.send(request)
        content = await response.json()
        # httpbin returns form we passed in the response
        content.show("form")


    asyncio.run(main())
    ```
    ```json
    {"foo": "bar"}
    ```

It supports a few standart python types so form like this:
```python
form={
    "array": ["bar", -123, False],
    "nothing": None,
    "float": 6.332,
    "boolean": True
}
```

Will produce such form string
```url
nothing=null&boolean=1&array=bar&array=-123&array=0&float=6.332
```
