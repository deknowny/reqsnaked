import asyncio

import reqsnaked

async def main():
    client = reqsnaked.Client()
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything",
        bearer_auth="0x0x0x"
    )
    response = await client.send(request)
    body = await response.json()
    body.show()


asyncio.run(main())
