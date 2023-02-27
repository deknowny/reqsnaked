# To Run image locally use
# docker run -p 80:80 -p 443:443 simonkowallik/httpbin:nginx

import asyncio
import ssl

import pytest
import aiohttp
import httpx
import reqsnaked


ssl_ctx = ssl.SSLContext()
HTTPBIN_URL = "https://localhost"


@pytest.fixture(scope="session")
def httpx_client():
    return httpx.Client()


def test_aiohttp_x10_linear(benchmark):
    @benchmark
    def inner():
        async def task():
            async with aiohttp.ClientSession() as session:
                for _ in range(10):
                    async with session.get(HTTPBIN_URL + "/anything", ssl=ssl_ctx) as response:
                        await response.json()

        asyncio.run(task())


def test_reqsnaked_x10_linear(benchmark):
    @benchmark
    def inner():
        async def task():
            request = reqsnaked.Request(
                "GET", HTTPBIN_URL + "/anything"
            )
            client = reqsnaked.Client(danger_accept_invalid_certs=True)
            for _ in range(10):
                response = await client.send(request)
                await response.json()

        asyncio.run(task())