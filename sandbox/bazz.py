import asyncio
import datetime
import os
import ssl

import aiohttp
import reqsnaked


ctx = ssl.SSLContext()

async def main():
    client = reqsnaked.AsyncClient()
    request_credentials = reqsnaked.Request(
        "GET",
        "https://api.vk.com/method/users.get",
        query={
            "access_token": os.getenv("GROUP_TOKEN"),
            "user_ids": "durov",
            "v": "5.131",
        },
    )
    for _ in range(10):
        start_time = datetime.datetime.now()
        response = await client.request(request_credentials)
        await response.json()
        print("reqsnaked:", datetime.datetime.now() - start_time)

    async with aiohttp.ClientSession() as session:
        for _ in range(10):
            start_time = datetime.datetime.now()
            response = await session.get(
                "https://api.vk.com/method/users.get",
                params={
                    "access_token": os.getenv("GROUP_TOKEN"),
                    "user_ids": "durov",
                    "v": "5.131",
                },
                ssl=ctx
            )
            await response.json()
            print("aiohttp:", datetime.datetime.now() - start_time)


asyncio.run(main())
