import typing
import dataclasses
import datetime
import urllib.parse
import http


@dataclasses.dataclass
class Part:
    name: str
    value: typing.Union[str, bytes]
    filename: str
    mime: str


@dataclasses.dataclass
class Multipart:
    parts: typing.List[Part]


@dataclasses.dataclass
class Request:
    method: typing.Union[str, "http.HTTPMethod"]
    url: typing.Union[str, urllib.parse.ParseResult]
    headers: typing.Optional[typing.Dict[str, str]] = None
    form: typing.Optional[
        typing.Dict[
            str, typing.Union[
                str, int, bool, None, typing.List[typing.Union[str, int, bool, None]]
            ]
        ]
    ] = None
    json: typing.Optional[typing.Any] = None
    bearer_auth: typing.Optional[str] = None
    body: typing.Optional[bytes] = None
    timeout: typing.Optional[typing.Union[int, datetime.timedelta]] = None
    multipart: typing.Optional[Multipart] = None
    username: typing.Optional[str] = None
    password: typing.Optional[str] = None


@dataclasses.dataclass
class Response:
    status: http.HTTPStatus
    # TODO


@dataclasses.dataclass
class Client:
    user_agent: typing.Optional[str] = None
    headers: typing.Optional[typing.Dict[str, str]] = None
    store_cookie: typing.Optional[bool] = None
    max_allowed_redirects: typing.Optional[int] = None
