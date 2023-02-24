
import asyncio
import os
import time
from typing import List, Any, Awaitable, Callable

import aiohttp
import reqsnaked


def get_reqsnaked_request(user_ids: List[str]) -> reqsnaked.Request:
    return reqsnaked.Request(
        "GET",
        "http://0.0.0.0:80/anything",
        query={
            "access_token": os.getenv("GROUP_TOKEN"),
            "user_ids": ",".join(user_ids),
            "v": "5.131",
        },
    )


async def reqsnaked_check(
    client: reqsnaked.AsyncClient,
    request: reqsnaked.Request,
    amount: int,
):
    for _ in range(amount):
        response = await client.request(request)
        await response.json()


def get_aiohttp_request(user_ids: List[str]):
    return dict(
        url="http://0.0.0.0:80/anything",
        params={
            "access_token": os.getenv("GROUP_TOKEN"),
            "user_ids": ",".join(user_ids),
            "v": "5.131",
        }
    )


async def aiohttp_check(
    client: aiohttp.ClientSession,
    amount: int,
    request: dict
):

    for _ in range(amount):
        response = await client.get(**request)
        await response.json()


async def _timeit(func: Callable[[Any, Any], Awaitable[Any]], **kwargs):
    start_time = time.monotonic()
    return await func(**kwargs), round(time.monotonic() - start_time, 6)


async def get_preprared_clients(user_ids: List[str]):
    reqsnaked_client = reqsnaked.AsyncClient()
    await reqsnaked_client.request(get_reqsnaked_request(user_ids))

    aiohttp_client = aiohttp.ClientSession()
    await aiohttp_client.get(**get_aiohttp_request(user_ids))

    return {"reqsnaked": reqsnaked_client, "aiohttp": aiohttp_client}


async def main():

    test_values = tuple(
        [
            [str(1)],
            [str(number) for number in range(1, 6)],
            [str(number) for number in range(1, 11)],
            [str(number) for number in range(1, 51)],
            [str(number) for number in range(1, 101)]
        ]
    )
    clients = await get_preprared_clients(test_values[0])
    for index, value in enumerate(test_values, start=1):

        _, aio_ex_time = await _timeit(
            aiohttp_check,
            request=get_aiohttp_request(value),
            amount=20,
            client=clients["aiohttp"]
        )
        _, req_ex_time = await _timeit(
            reqsnaked_check,
            request=get_reqsnaked_request(value),
            amount=20,
            client=clients["reqsnaked"]
        )
        winner = "AIOHTTP" if aio_ex_time < req_ex_time else "REQSNAKED"
        print(
            f"\n\n\n\nTEST{index}, 20 times with {len(value)} user_ids:"
            f"\n\n--AIOHTTP: {aio_ex_time} (AVG: {aio_ex_time/20})"
            f"\n--REQSNAKED: {req_ex_time} (AVG: {req_ex_time/20})"
            f"\n\nWINNER: {winner}"
        )


asyncio.run(main())
