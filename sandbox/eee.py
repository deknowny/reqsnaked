import asyncio

import reqsnaked

async def main():
    client = reqsnaked.Client()
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/image/jpeg",
    )
    response = await client.send(request)
    stream = response.to_stream()
    while chunk := await stream.gnaw():
        print(chunk)

asyncio.run(main())
