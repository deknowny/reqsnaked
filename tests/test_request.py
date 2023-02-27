import http

import reqsnaked
import pytest


@pytest.mark.asyncio
async def test_simple(client: reqsnaked.Client):
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything"
    )
    response = await client.send(request)
    assert response.status == http.HTTPStatus.OK


@pytest.mark.asyncio
async def test_query(client: reqsnaked.Client):
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything",
        query={
            "array": ["bar", -123, False],
            "nothing": None,
            "float": 6.332,
            "boolean": True
        }
    )
    response = await client.send(request)
    body = await response.json()
    assert body.query("args") == {
        "array": ["bar", "-123", "0"],
        "nothing": "null",
        "float": "6.332",
        "boolean": "1"
    }


@pytest.mark.asyncio
async def test_form(client: reqsnaked.Client):
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything",
        form={
            "array": ["bar", -123, False],
            "nothing": None,
            "float": 6.332,
            "boolean": True
        }
    )
    response = await client.send(request)
    body = await response.json()
    assert body.query("form") == {
        "array": ["bar", "-123", "0"],
        "nothing": "null",
        "float": "6.332",
        "boolean": "1"
    }


@pytest.mark.asyncio
async def test_json(client: reqsnaked.Client):
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything",
        json={
            "array": ["bar", -123, False],
            "nothing": None,
            "float": 6.332,
            "boolean": True
        }
    )
    response = await client.send(request)
    body = await response.json()
    assert body.query("json") == {
        "array": ["bar", -123, False],
        "nothing": None,
        "float": 6.332,
        "boolean": True
    }


@pytest.mark.asyncio
async def test_bearer_auth(client: reqsnaked.Client):
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything",
        bearer_auth="0x0x0x"
    )
    response = await client.send(request)
    body = await response.json()
    assert body.query("headers", "Authorization") == "Bearer 0x0x0x"


@pytest.mark.asyncio
async def test_basic_auth(client: reqsnaked.Client):
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything",
        username="John",
        password="D000eee"
    )
    response = await client.send(request)
    body = await response.json()
    assert body.query("headers", "Authorization") == "Basic Sm9objpEMDAwZWVl"


@pytest.mark.asyncio
async def test_multipart(client: reqsnaked.Client):
    request = reqsnaked.Request(
        "GET", "https://httpbin.org/anything",
        multipart=reqsnaked.Multipart(
            reqsnaked.Part("foo", b"01010", filename="foo.txt"),
            reqsnaked.Part("bar", "abcdef", filename="bar.txt")
        )
    )
    response = await client.send(request)
    body = await response.json()
    assert body.query("files") == {
        "bar": "abcdef",
        "foo": "01010"
    }
