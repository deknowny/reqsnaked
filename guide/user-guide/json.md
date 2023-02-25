## Send `application/json`
Simply to add json to request, you need to pass a dict with params into `reqsnaked.Request` construction.

```python
request = reqsnaked.Request(
    "POST", "https://httpbin.org/anything",
    json={"foo": "bar"}
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
            json={"foo": "bar"}
        )
        response = await client.send(request)
        content = await response.json()
        # httpbin returns json we passed in the response
        content.show("json")


    asyncio.run(main())
    ```
    ```json
    {"foo": "bar"}
    ```

It supports all python types that a normally could be dumped using built-in `json` module. So this argument
```python
json={
    "array": ["bar", -123, False],
    "nothing": None,
    "float": 6.332,
    "boolean": True
}
```

will produce such JSON
```json
{
    "array": [
      "bar",
      -123,
      false
    ],
    "boolean": true,
    "float": 6.332,
    "nothing": null
}
```
## Read JSON from body
JSON could be both read and parse using `Response.json()` method. It returns a special `LazyJSON` object:

```python
client = reqsnaked.Client()
request = reqsnaked.Request(
    "POST", "https://httpbin.org/json",
)
response = await client.send(request)
data = await response.json()
data.show()
```
```json
{
  "slideshow": {
    "author": "Yours Truly",
    "date": "date of publication",
    "slides": [
      {
        "title": "Wake up to WonderWidgets!",
        "type": "all"
      },
      {
        "items": [
          "Why <em>WonderWidgets</em> are great",
          "Who <em>buys</em> WonderWidgets"
        ],
        "title": "Overview",
        "type": "all"
      }
    ],
    "title": "Sample Slide Show"
  }
}
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client()
        request = reqsnaked.Request(
            "POST", "https://httpbin.org/json",
        )
        response = await client.send(request)
        data = await response.json()
        data.show()


    asyncio.run(main())
    ```
    ```json
    {"foo": "bar"}
    ```

It's called "lazy" because it does not load everything into Python types immidiately. To access something use `.query(...)` method, which accepts `#!python *args` that uses chainly to get an object:

```python
client = reqsnaked.Client()
request = reqsnaked.Request(
    "POST", "https://httpbin.org/json",
)
response = await client.send(request)
data = await response.json()
print(data.query("slideshow", "slides", 0, "title"))
```
```
"Wake up to WonderWidgets!"
```
??? Abstract "Full code preview"
    ```python
    import asyncio

    import reqsnaked


    async def main():
        client = reqsnaked.Client()
        request = reqsnaked.Request(
            "POST", "https://httpbin.org/json",
        )
        response = await client.send(request)
        data = await response.json()
        print(data.query("slideshow", "slides", 0, "title"))


    asyncio.run(main())
    ```
    ```json
    {"foo": "bar"}
    ```
It's the same as you would use `#!python data["slideshow"]["slides"][0]["title"]` with an ordinary dict. In this case only string `#!python "Wake up to WonderWidgets!"` will be loaded to python types, other will be stored using Rust cheap types.

!!! Tip
    The same "chaining" works for `.show(...)`


!!! Failure
    You cannot read from body twice
