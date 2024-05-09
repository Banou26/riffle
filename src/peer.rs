use crate::{bitfield::BitField, tracker::TrackerPeer, utils::IpAddr};

/**
 * Overview
 * The peer protocol facilitates the exchange of pieces as described in the 'metainfo file.
 * 
 * Note here that the original specification also used the term "piece" when describing the peer protocol, but as a different term than "piece" in the metainfo file. For that reason, the term "block" will be used in this specification to describe the data that is exchanged between peers over the wire.
 * 
 * A client must maintain state information for each connection that it has with a remote peer:
 * 
 * choked: Whether or not the remote peer has choked this client. When a peer chokes the client, it is a notification that no requests will be answered until the client is unchoked. The client should not attempt to send requests for blocks, and it should consider all pending (unanswered) requests to be discarded by the remote peer.
 * interested: Whether or not the remote peer is interested in something this client has to offer. This is a notification that the remote peer will begin requesting blocks when the client unchokes them.
 * Note that this also implies that the client will also need to keep track of whether or not it is interested in the remote peer, and if it has the remote peer choked or unchoked. So, the real list looks something like this:
 * 
 * am_choking: this client is choking the peer
 * am_interested: this client is interested in the peer
 * peer_choking: peer is choking this client
 * peer_interested: peer is interested in this client
 * Client connections start out as "choked" and "not interested". In other words:
 * 
 * am_choking = 1
 * am_interested = 0
 * peer_choking = 1
 * peer_interested = 0
 * A block is downloaded by the client when the client is interested in a peer, and that peer is not choking the client. A block is uploaded by a client when the client is not choking a peer, and that peer is interested in the client.
 * 
 * It is important for the client to keep its peers informed as to whether or not it is interested in them. This state information should be kept up-to-date with each peer even when the client is choked. This will allow peers to know if the client will begin downloading when it is unchoked (and vice-versa).
 */

/**
 * Data Types
 * Unless specified otherwise, all integers in the peer wire protocol are encoded as four byte big-endian values. This includes the length prefix on all messages that come after the handshake.
 */

/**
 * Message flow
 * The peer wire protocol consists of an initial handshake. After that, peers communicate via an exchange of length-prefixed messages. The length-prefix is an integer as described above.
 */


#[derive(Debug)]
pub struct PeerWire {
  info: TrackerPeer,

  peer_id: Option<String>,
  ip: IpAddr,
  port: u16,

  am_choking: bool,
  am_interested: bool,

  peer_choking: bool,
  peer_interested: bool,

  peer_bitfield: BitField,
  bitfield: BitField,
}

/**
 * The handshake is a required message and must be the first message transmitted by the client. It is (49+len(pstr)) bytes long.
 * handshake: <pstrlen><pstr><reserved><info_hash><peer_id>
 * pstrlen: string length of <pstr>, as a single raw byte
 * pstr: string identifier of the protocol
 * reserved: eight (8) reserved bytes. All current implementations use all zeroes. Each bit in these bytes can be used to change the behavior of the protocol. An email from Bram suggests that trailing bits should be used first, so that leading bits may be used to change the meaning of trailing bits.
 * info_hash: 20-byte SHA1 hash of the info key in the metainfo file. This is the same info_hash that is transmitted in tracker requests.
 * peer_id: 20-byte string used as a unique ID for the client. This is usually the same peer_id that is transmitted in tracker requests (but not always e.g. an anonymity option in Azureus).
 * In version 1.0 of the BitTorrent protocol, pstrlen = 19, and pstr = "BitTorrent protocol".
 * The initiator of a connection is expected to transmit their handshake immediately. The recipient may wait for the initiator's handshake, if it is capable of serving multiple torrents simultaneously (torrents are uniquely identified by their infohash). However, the recipient must respond as soon as it sees the info_hash part of the handshake (the peer id will presumably be sent after the recipient sends its own handshake). The tracker's NAT-checking feature does not send the peer_id field of the handshake.
 * If a client receives a handshake with an info_hash that it is not currently serving, then the client must drop the connection.
 * If the initiator of the connection receives a handshake in which the peer_id does not match the expected peerid, then the initiator is expected to drop the connection. Note that the initiator presumably received the peer information from the tracker, which includes the peer_id that was registered by the peer. The peer_id from the tracker and in the handshake are expected to match.
 */
#[derive(Debug)]
pub struct Handshake {
  pstrlen: u8,

  /** In version 1.0 of the BitTorrent protocol, pstrlen = 19, and pstr = "BitTorrent protocol". */
  pstr: String,
  // pstr: [u8; 19],
  reserved: [u8; 8],
  info_hash: [u8; 20],

  peer_id: [u8; 20],
}

/**
 * All of the remaining messages in the protocol take the form of <length prefix><message ID><payload>. The length prefix is a four byte big-endian value. The message ID is a single decimal byte. The payload is message dependent.
 */

#[derive(Debug)]
pub enum MessageId {
  Choke = 0,
  Unchoke = 1,
  Interested = 2,
  NotInterested = 3,
  Have = 4,
  /** 1 + x */
  Bitfield = 5,
  Request = 6,
  /** 9 + x */
  Piece = 7,
  Cancel = 8,
  Port = 9,
}

#[derive(Debug)]
pub struct Message {
  length_prefix: u32,
  message_id: MessageId,
  payload: Vec<u8>
}

/**
 * The keep-alive message is a message with zero bytes, specified with the length prefix set to zero. There is no message ID and no payload. Peers may close a connection if they receive no messages (keep-alive or any other message) for a certain period of time, so a keep-alive message must be sent to maintain the connection alive if no command have been sent for a given amount of time. This amount of time is generally two minutes.
 */
#[derive(Debug)]
pub struct KeepAlive {
  length_prefix: u32,
}

/**
 * The choke message is fixed-length and has no payload.
 * choke: <len=0001><id=0>
 */
#[derive(Debug)]
pub struct Choke {
  length_prefix: u32,
  message_id: MessageId,
}

/**
 * The unchoke message is fixed-length and has no payload.
 * unchoke: <len=0001><id=1>
 */
#[derive(Debug)]
pub struct Unchoke {
  length_prefix: u32,
  message_id: MessageId,
}

/**
 * The interested message is fixed-length and has no payload.
 * interested: <len=0001><id=2>
 */
#[derive(Debug)]
pub struct Interested {
  length_prefix: u32,
  message_id: MessageId,
}

/**
 * The not interested message is fixed-length and has no payload.
 * not interested: <len=0001><id=3>
 */
#[derive(Debug)]
pub struct NotInterested {
  length_prefix: u32,
  message_id: MessageId,
}

/**
 * The have message is fixed length. The payload is the zero-based index of a piece that has just been successfully downloaded and verified via the hash.
 * have: <len=0005><id=4><piece index>
 */
#[derive(Debug)]
pub struct Have {
  length_prefix: u32,
  message_id: MessageId,
  piece_index: u32,
}

/**
 * The bitfield message may only be sent immediately after the handshaking sequence is completed, and before any other messages are sent. It is optional, and need not be sent if a client has no pieces.
 * bitfield: <len=0001+X><id=5><bitfield>
 * The bitfield message is variable length, where X is the length of the bitfield. The payload is a bitfield representing the pieces that have been successfully downloaded. The high bit in the first byte corresponds to piece index 0. Bits that are cleared indicated a missing piece, and set bits indicate a valid and available piece. Spare bits at the end are set to zero.
 */
#[derive(Debug)]
pub struct Bitfield {
  length_prefix: u32,
  message_id: MessageId,
  bitfield: Vec<u8>,
}

/**
 * The request message is fixed length, and is used to request a block. The payload contains the following information:
 * request: <len=0013><id=6><index><begin><length>
 * index: integer specifying the zero-based piece index
 * begin: integer specifying the zero-based byte offset within the piece
 * length: integer specifying the requested length.
 * The request message is fixed length, and is used to request a block. The payload contains the following information:
 * request: <len=0013><id=6><index><begin><length>
 * index: integer specifying the zero-based piece index
 * begin: integer specifying the zero-based byte offset within the piece
 * length: integer specifying the requested length.
 */
#[derive(Debug)]
pub struct Request {
  length_prefix: u32,
  message_id: MessageId,
  index: u32,
  begin: u32,
  length: u32,
}

/**
 * The piece message is variable length, where X is the length of the block. The payload contains the following information:
 * piece: <len=0009+X><id=7><index><begin><block>
 * index: integer specifying the zero-based piece index
 * begin: integer specifying the zero-based byte offset within the piece
 * block: block of data, which is a subset of the piece specified by index.
 * The piece message is variable length, where X is the length of the block. The payload contains the following information:
 * piece: <len=0009+X><id=7><index><begin><block>
 * index: integer specifying the zero-based piece index
 * begin: integer specifying the zero-based byte offset within the piece
 * block: block of data, which is a subset of the piece specified by index.
 */
#[derive(Debug)]
pub struct Piece {
  length_prefix: u32,
  message_id: MessageId,
  index: u32,
  begin: u32,
  block: Vec<u8>,
}

/**
 * The cancel message is fixed length, and is used to cancel a block request. The payload is identical to that of the "request" message.
 * cancel: <len=0013><id=8><index><begin><length>
 * index: integer specifying the zero-based piece index
 * begin: integer specifying the zero-based byte offset within the piece
 * length: integer specifying the requested length.
 */
#[derive(Debug)]
pub struct Cancel {
  length_prefix: u32,
  message_id: MessageId,
  index: u32,
  begin: u32,
  length: u32,
}

/**
 * The port message is sent by newer versions of the Mainline that implements a DHT tracker. The listen port is the port this peer's DHT node is listening on. This peer should be inserted in the local routing table (if DHT tracker is supported).
 * port: <len=0003><id=9><listen-port>
 * listen-port is a 16-bit big-endian value.
 */
#[derive(Debug)]
pub struct Port {
  length_prefix: u32,
  message_id: MessageId,
  listen_port: u16,
}
