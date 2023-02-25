import asyncio

import reqsnaked

async def main():
    client = reqsnaked.Client()
    request = reqsnaked.Request(
        "POST", "https://httpbin.org/anything",
        json={
            "array": ["bar", -123, False],
            "nothing": None,
            "float": 6.332,
            "boolean": True
        }
    )
    response = await client.send(request)
    content = await response.json()
    # httpbin returns query string we passed in the response
    content.show()

asyncio.run(main())
