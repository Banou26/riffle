
import type { Instance as TorrentFile } from 'parse-torrent'

import { createMachine } from 'xstate'

export const torrent = createMachine({
  id: 'torrent',
  initial: 'checking',
  context: {
    torrentFile: undefined as unknown as TorrentFile,
  } as { torrentFile: TorrentFile },
  states: {
    checking: {},
    downloading: {},
    seeding: {},
    completed: {},
    paused: {},
  },
})

export type Torrent = typeof torrent
