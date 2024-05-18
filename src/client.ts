import type { Torrent } from './torrent'

import { assign, createActor, createMachine } from 'xstate'

export const TorrentClient = createMachine({
  id: 'torrent-client',
  initial: 'idle',
  context: ({ input }: { input: { torrents: Torrent[] } }) => ({
    torrents: input.torrents
  }),
  invoke: {
    id: 'addTorrent',
    src: 'fetchUser',
    input: ({ context: { userId } }) => ({ userId }),
    onDone: {
      target: 'success',
      actions: assign({ user: ({ event }) => event.output }),
    },
    onError: {
      target: 'failure',
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

  }
}
