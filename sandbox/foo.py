import asyncio

import reqsnaked



async def main():
    client = reqsnaked.AsyncClient(
        user_agent="Reqsnaked/1.0",
        headers={"X-Foo": "bar"}
    )
    response = await client.request(
        reqsnaked.Request(
            "POST",
            "https://httpbin.org/anything",
            query={"foo": "bar"},
            timeout=datetime.timedelta(seconds=30),
            basic_auth=reqsnaked.BasicAuth("John", "D0eeee")
        )
    )
    print(response.status.code)
    data = await response.json()
    print(data.select())


asyncio.run(main())
