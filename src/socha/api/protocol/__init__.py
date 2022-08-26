"""
The protocol module contains the classes that define the protocol used by the Software-Challenge for communication
between the server and the client. It contains all necessary classes that represent the xml messages that sends the
server and the client.
"""
from src.socha.api.protocol.protocol_packet import *

__all__ = [
    'ProtocolPacket',
    'ResponsePacket',
    'LobbyRequest',
    'AdminLobbyRequest'
]
