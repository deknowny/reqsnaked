## Basic auth
To add basic auth to your request, use `#!python username=` and `#!python password=` params:

```python
client = reqsnaked.Client()
request = reqsnaked.Request(
    "POST", "https://httpbin.org/anything",
    username="John",
    password="D000eee"  # It could be optional
)
response = await client.send(request)
data = await response.json()
print(data.query("headers", "Authorization"))
```
```
Basic Sm9objpEMDAwZWVl
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client()
        request = reqsnaked.Request(
            "POST", "https://httpbin.org/anything",
            username="John",
            password="D000eee"  # Could be optional
        )
        response = await client.send(request)
        data = await response.json()
        print(data.query("headers", "Authorization"))


    asyncio.run(main())
    ```


## Bearer auth
To add bearer auth to your request, use `#!python bearer=` param:

```python
client = reqsnaked.Client()
request = reqsnaked.Request(
    "POST", "https://httpbin.org/anything",
    bearer_auth="fizzbazzeggg",
)
response = await client.send(request)
data = await response.json()
print(data.query("headers", "Authorization"))
```
```
Bearer fizzbazzeggg
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client()
        request = reqsnaked.Request(
            "POST", "https://httpbin.org/anything",
            bearer_auth="fizzbazzeggg",
        )
        response = await client.send(request)
        data = await response.json()
        print(data.query("headers", "Authorization"))

    asyncio.run(main())
    ```
