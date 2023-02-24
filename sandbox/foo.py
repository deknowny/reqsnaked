import asyncio
import datetime
import urllib.parse
import http

import reqsnaked


async def main():
    client = reqsnaked.Client(
        user_agent="Reqsnaked/1.0",
        headers={"X-Foo": "bar"}
    )
    response = await client.send(
        reqsnaked.Request(
            "POST",
            urllib.parse.urlparse("https://httpbin.org/anything"),
            multipart=reqsnaked.Multipart(
                reqsnaked.Part("foo", b"01010101", filename="foo.txt", mime="text/plain")
            ),
            timeout=datetime.timedelta(seconds=30),
        )
    )
    print(response.status.description)
    print(response.version)
    data = await response.json()
    print(data.select())


asyncio.run(main())
