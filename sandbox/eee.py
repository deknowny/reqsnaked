import asyncio

import reqsnaked

async def main():
    client = reqsnaked.Client()
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/bytes/10",
    )
    response = await client.send(request)
    data = await response.read()
    print(data.as_bytes())

asyncio.run(main())
