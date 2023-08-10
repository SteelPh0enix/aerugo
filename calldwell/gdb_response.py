from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import Any, Dict, Iterator, List, Optional, Union


@dataclass
class GDBResponse:
    """Structure representing GDB response."""

    Payload = Union[Dict[Any, Any], List[Any], str]

    class Type(Enum):
        RESULT = "result"
        NOTIFY = "notify"
        CONSOLE = "console"
        LOG = "log"
        OUTPUT = "output"
        TARGET = "target"
        DONE = "done"

        def __str__(self) -> str:
            return self.value

        def __repr__(self) -> str:
            return str(self)

    class Stream(Enum):
        STDOUT = "stdout"
        STDIN = "stdin"
        STDERR = "stderr"

        def __str__(self) -> str:
            return self.value

        def __repr__(self) -> str:
            return str(self)

    message: Optional[str]
    """Message is usually a human-readable string."""
    payload: Optional[Payload]
    """Payload can be either a string, list or a dict."""
    token: Optional[Any]
    type: Type
    """Response's type, always present."""
    stream: Optional[Stream]
    """Response's stream, always present in GDB responses, but left as `Optional` to allow creating
    GDBResponses for comparison."""

    def is_similar(self, other: GDBResponse) -> bool:
        """Check if another response is 'similar' to current one.
        Similarity criteria are:
        - Both responses must have the same type to be similar, AND
        - If `other` response has message, then current one must have the same message
          to be similar, AND
        - If `other` response has payload, then current one must have the same payload

        In other words, type must be equal, and message/payload must be equal if set in `other`.
        """
        if self.type != other.type:
            return False

        if other.message is not None and self.message != other.message:
            return False

        if other.payload is not None and self.payload != other.payload:
            return False

        return True

    def unescaped_payload(self, strip_whitespace: bool = True) -> str:
        """If payload is a string, it probably contains escaped special characters.
        Use this function to get unescaped version of payload string.

        `strip_whitespace` controls whether returned string should be stripped
        out of whitespace at it's beginning and end, or not. By default, returned strings
        are stripped.
        """
        payload = str(self.payload).replace("\\n", "\n").replace("\\t", "\t").replace('\\"', '"')
        if strip_whitespace:
            return payload.strip()
        return payload

    @staticmethod
    def with_message(type: Type, message: str) -> GDBResponse:
        """Returns a GDBResponse with only `type` and `message` fields set.
        Use this function to create a response object for `is_similar` comparison."""
        return GDBResponse(message=message, type=type, payload=None, token=None, stream=None)

    @staticmethod
    def with_payload(type: Type, payload: Payload) -> GDBResponse:
        """Returns a GDBResponse with only `type` and `payload` fields set.
        Use this function to create a response object for `is_similar` comparison."""
        return GDBResponse(payload=payload, type=type, message=None, token=None, stream=None)


class GDBResponsesList:
    """Class representing a list of GDB responses.

    Besides providing a list-like access to individual responses, it also provides some commonly
    performed operations on them, like checking if a specific type of response is on the list,
    or concatenating the console messages into singular string.
    """

    def __init__(self, responses: List[GDBResponse]) -> None:
        """Initialize the list with responses received from GDB"""
        self.items = responses

    def of_type(self, response_type: GDBResponse.Type) -> GDBResponsesList:
        """Return all the responses on the list that have specified type"""
        return GDBResponsesList(list(filter(lambda response: response.type == response_type, self)))

    def results(self) -> GDBResponsesList:
        """Returns a list containing only `result`-type messages"""
        return self.of_type(GDBResponse.Type.RESULT)

    def notifications(self) -> GDBResponsesList:
        """Returns a list containing only `notify`-type messages"""
        return self.of_type(GDBResponse.Type.NOTIFY)

    def console(self) -> GDBResponsesList:
        """Returns a list containing only `console`-type messages"""
        return self.of_type(GDBResponse.Type.CONSOLE)

    def logs(self) -> GDBResponsesList:
        """Returns a list containing only `log`-type messages"""
        return self.of_type(GDBResponse.Type.LOG)

    def outputs(self) -> GDBResponsesList:
        """Returns a list containing only `output`-type messages"""
        return self.of_type(GDBResponse.Type.OUTPUT)

    def target(self) -> GDBResponsesList:
        """Returns a list containing only `target`-type messages"""
        return self.of_type(GDBResponse.Type.TARGET)

    def payload_string_list(self, unescape: bool = True) -> List[str]:
        """Returns a list of stringified payloads from all responses.

        # Parameters
        * `escape` - If `True`, escaped characters in payloads will be unescaped to
                     produce human-readable string.
        """
        if unescape:
            payloads = [response.unescaped_payload(strip_whitespace=False) for response in self]
        else:
            payloads = [str(response.payload) for response in self]

        return payloads

    def payload_string(self, separator: str = "", unescape: bool = True) -> str:
        """Returns a single stringified payload from all responses. Returned string is stripped
        of whitespace at the beginning and the end.

        # Parameters
        * `separator` [str] - A separator inserter between each payload on the list.
        * `escape` [bool] - If `True`, escaped characters in payloads will be unescaped to
                            produce human-readable string.
        """

        return separator.join(self.payload_string_list(unescape)).strip()

    def extend(self, other: GDBResponsesList):
        """Add items from different response list to current one."""
        self.items.extend(other.items)

    def __contains__(self, expected: GDBResponse) -> bool:
        """Returns `True` if any item in response list is similar to expected response.
        To see how the items are compared, see `GDBResponse.is_similar()`."""
        return any(response.is_similar(expected) for response in self)

    def __len__(self):
        """Return amount of elements on the list"""
        return len(self.items)

    def __getitem__(self, key: int) -> GDBResponse:
        """Return a specific element of the list by it's index"""
        return self.items[key]

    def __iter__(self) -> Iterator[GDBResponse]:
        """Return an iterator over the elements on the list"""
        return self.items.__iter__()

    def __str__(self) -> str:
        return "\n".join([f"[{i}] {response}" for i, response in enumerate(self)])

    def __repr__(self) -> str:
        return str(self)
