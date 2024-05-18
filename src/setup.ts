import { setup } from 'xstate';
import { TorrentClient } from './client'
import { Peer } from './peer'
import { Piece } from './piece'
import { Torrent } from './torrent'

export default setup({
  actors: {
    TorrentClient,
    Peer,
    Piece,
    Torrent
  }
})

