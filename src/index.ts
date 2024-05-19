import { readFile } from 'fs/promises'
import parseTorrent from 'parse-torrent'

import { makeClient } from './client'

const torrentClient = makeClient()

console.log('torrentClient', torrentClient)

const torrentFile = await readFile('torrent_test.torrent').then(parseTorrent)

torrentClient.addTorrent(torrentFile)
