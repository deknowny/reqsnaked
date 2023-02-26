import http

import pytest
import reqsnaked



@pytest.mark.asyncio
async def test_mess():
    client = reqsnaked.Client(
        user_agent="Reqsnaked",
        headers={"X-Foo": "Bar"}
    )

    request = reqsnaked.Request(
        "POST", "https://httpbin.org/anything",
        query={"foo": "bar"},
        form={"egg": "brain"}
    )

    response = await client.send(request)
    assert response.status == http.HTTPStatus.OK

    body = await response.json()
    assert body.query("args") == {"foo": "bar"}
    assert body.query("form") == {"egg": "brain"}
    assert body.query("headers", "X-Foo") == "Bar"
    assert body.query("headers", "User-Agent") == "Reqsnaked"