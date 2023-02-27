import http

import reqsnaked
import pytest


@pytest.mark.asyncio
async def test_simple():
    client = reqsnaked.Client()
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything"
    )
    response = await client.send(request)
    assert response.status == http.HTTPStatus.OK


@pytest.mark.asyncio
async def test_default_headers():
    client = reqsnaked.Client(headers={"X-Foo": "Bar"})
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything"
    )
    response = await client.send(request)
    body = await response.json()
    assert body.query("headers", "X-Foo") == "Bar"


@pytest.mark.asyncio
async def test_default_user_agent():
    client = reqsnaked.Client(user_agent="Reqsnaked/1.0")
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything"
    )
    response = await client.send(request)
    body = await response.json()
    assert body.query("headers", "User-Agent") == "Reqsnaked/1.0"
