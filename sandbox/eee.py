import asyncio
import datetime

import reqsnaked


async def main():
    client = reqsnaked.Client(
        user_agent="Reqsnaked/1.0",
        headers={"X-Foo": "bar"},
        store_cookie=True
    )
    request = reqsnaked.Request(
        "POST", "https://httpbin.org/anything",
        multipart=reqsnaked.Multipart(
            reqsnaked.Part(
                "foo", b"01010101",
                filename="foo.txt",
                mime="text/plain"
            )
        ),
        query={"foo": "bar"},
        headers={"X-Bar": "foo"},
        timeout=datetime.timedelta(seconds=30),
    )
    response = await client.send(request)
    print(response.status)
    data = await response.json()
    data.show()


asyncio.run(main())