import asyncio
import datetime

import reqsnaked


async def main():
    client = reqsnaked.AsyncClient(
        user_agent="Reqsnaked/1.0",
        headers={"X-Foo": "bar"}
    )
    response = await client.request(
        reqsnaked.Request(
            "GET",
            "https://httpbin.org/image/jpeg",
            timeout=datetime.timedelta(seconds=30),
        )
    )
    print(response.status.code)
    stream = response.to_stream()
    while chunk := await stream.gnaw():
        print(chunk)


asyncio.run(main())
