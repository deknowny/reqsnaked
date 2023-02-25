import asyncio

import reqsnaked

async def main():
    client = reqsnaked.Client(headers={"X-Foo": "Bar"})
    request = reqsnaked.Request("POST", "https://httpbin.org/anything", query={"foo": "bar"})
    response = await client.send(request)
    content = await response.json()
    content.show()
    print(response.headers["DATE"])

asyncio.run(main())
