import asyncio
import datetime

import reqsnaked


async def main():
    client = reqsnaked.Client(
        user_agent="Reqsnaked/1.0",
        headers={"X-Foo": "bar"},
    )
    response = await client.send(
        reqsnaked.Request(
            "POST",
            "https://httpbin.org/anything",
            multipart=reqsnaked.Multipart(
                reqsnaked.Part("foo", b"01010101", filename="foo.txt", mime="text/plain")
            ),
            query={"foo": "bar"},
            headers={"X-Bar": "foo"},
            timeout=datetime.timedelta(seconds=30),
        )
    )
    print(response.status)
    data = await response.json()
    print(data.select())


asyncio.run(main())
