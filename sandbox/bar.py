import asyncio
import dataclasses
import typing



@dataclasses.dataclass
class Stream:

    storage: typing.List[int] = dataclasses.field(default_factory=lambda: [1, 2, 3])

    async def read(self) -> typing.Optional[int]:
        if self.storage:
            return self.storage.pop()
        return None



async def main():
    stream = Stream()

    while chunk := await stream.read():
        print(chunk)


asyncio.run(main())
