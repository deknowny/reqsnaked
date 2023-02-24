
import asyncio
import datetime
import os
import ssl
import time

import aiohttp
import reqsnaked
import uvloop

uvloop.install()
ctx = ssl.SSLContext()


async def main():
    client = reqsnaked.AsyncClient()
    request_credentials = reqsnaked.Request(
        "POST",
        "http://0.0.0.0:80/anything",
        form={
            "access_token": os.getenv("GROUP_TOKEN"),
            "user_ids": ",".join([str(number) for number in range(100)]),
            "v": "5.131",
        },
    )
    aiohttp_credentials = dict(
        url="http://0.0.0.0:80/anything",
        data={
            "access_token": os.getenv("GROUP_TOKEN"),
            "user_ids": ",".join([str(number) for number in range(100)]),
            "v": "5.131",
        }
    )
    timings = []
    await client.request(request_credentials)
    print("enter")
    for i in range(100):
        start_time = time.time()
        async def task():
            response = await client.request(request_credentials)
            await response.json()
        await asyncio.gather(*[task() for _ in range(100)])
        timings.append(time.time() - start_time)
        print(f"+1/{i}\r")
    print("reqsnaked:", sum(timings) / len(timings))
    timings.clear()
    async with aiohttp.ClientSession() as session:
        for _ in range(100):
            start_time = time.time()
            async def task():
                response = await session.post(**aiohttp_credentials, ssl=ctx)
                await response.json()
            await asyncio.gather(*[task() for _ in range(100)])
            timings.append(time.time() - start_time)

    print("aiohttp:", sum(timings) / len(timings))


asyncio.run(main())
