import type { Instance as TorrentFile } from 'parse-torrent'

import type { Torrent } from './torrent'

import { assign, createActor, setup } from 'xstate'

import { torrent } from './torrent'

export const TorrentClient = setup({
  actors: {
    torrent,
  },
  actions: {
    addTorrent: assign({
      torrents: ({ spawn }) => [...torrents, torrentFile]
    })
  }
}).createMachine({
  id: 'torrent-client',
  initial: 'idle',
  context: ({ input }: { input: { torrents: Torrent[] } }) => ({
    torrents: input.torrents
  }),
  states: {
    idle: {}
  },
  invoke: {
    id: 'addTorrent',
    src: 'torrent',
    input: ({ context: { torrentFile } }: { context: { torrentFile: TorrentFile } }) => ({ torrentFile }),
    onDone: {
      target: '.idle',
      actions: assign({ user: ({ event }) => event.output }),
    },
    onError: {
      target: '.idle',
      actions: assign({ error: ({ event }) => event.error }),
    },
  }
})

export const makeClient = () => {
  const torrentClientActor = createActor(TorrentClient, {
    input: {
      torrents: []
    }
  })

  return {
    addTorrent: (torrentFile: TorrentFile) => {
      torrentClientActor.send({ type: 'addTorrent' })
    }
  }
}
