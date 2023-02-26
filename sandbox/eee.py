import asyncio

import reqsnaked

async def main():
    client = reqsnaked.Client(
        user_agent="Reqsnaked",
        headers={"X-Foo": "Bar"}
    )

    request = reqsnaked.Request(
        "POST", "https://httpbin.org/anything",
        query={"foo": "bar"},
        form={"foo": "bar"}
    )

    response = await client.send(request)
    body = await response.json()
    body.show()

asyncio.run(main())
