import typing
import dataclasses
import datetime
import urllib.parse
import http
import enum


QueryValue = typing.Union[
    str, int, bool, None, typing.List[
        typing.Union[str, int, bool, None]
    ]
]


class Bytes:
    def to_bytes(self) -> bytes:
        pass


class Stream:
    async def gnaw(self) -> typing.Optional[Bytes]:
        pass


class HTTPVersion(enum.Enum):
    HTTP_09 = enum.auto()
    HTTP_10 = enum.auto()
    HTTP_11 = enum.auto()
    HTTP_2 = enum.auto()
    HTTP_3 = enum.auto()

    def to_string(self) -> str:
        pass

@dataclasses.dataclass
class HeaderMap:
    def to_dict(self) -> typing.Dict[str, bytes]:
        pass

    def __getitem__(self, item: str) -> bytes:
        pass



@dataclasses.dataclass
class LazyJSON:
    def query(self, *keys: typing.Union[str, int]) -> typing.Any:
        pass

    def show(self, *keys: typing.Union[str, int]) -> None:
        pass


@dataclasses.dataclass
class Part:
    name: str
    value: typing.Union[str, bytes]
    filename: str
    mime: str


@dataclasses.dataclass
class Multipart:
    parts: typing.List[Part]

    def boundary(self) -> str:
        pass


@dataclasses.dataclass
class Request:
    method: typing.Union[str, "http.HTTPMethod"]
    url: typing.Union[str, urllib.parse.ParseResult]
    headers: typing.Optional[typing.Dict[str, str]] = None
    query: typing.Optional[typing.Dict[str, QueryValue]] = None
    form: typing.Optional[typing.Dict[str, QueryValue]] = None
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
    version: HTTPVersion
    headers: HeaderMap

    async def json(self) -> LazyJSON:
        pass

    async def read(self) -> Bytes:
        pass

    def to_stream(self) -> Stream:
        pass


@dataclasses.dataclass
class Client:
    user_agent: typing.Optional[str] = None
    headers: typing.Optional[typing.Dict[str, str]] = None
    store_cookie: typing.Optional[bool] = False
    max_allowed_redirects: typing.Optional[int] = 10

    async def send(self, requset) -> Response:
        pass
