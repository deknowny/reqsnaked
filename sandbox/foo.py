import asyncio

import reqsnaked



async def main():
    client = reqsnaked.aio.client.AsyncClient(
        user_agent="Reqsnaked/1.0",
        headers={"X-Foo": "bar"}
    )
    response = await client.request(
        reqsnaked.aio.request.AsyncRequest(
            reqsnaked.primitives.HTTPMethod.from_str("GET"),
            reqsnaked.primitives.URL.from_str("https://httpbin.org/headers")
        )
    )
    print(response.status.code)
    data = await response.json()
    print(data.select("headers"))


asyncio.run(main())
