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
