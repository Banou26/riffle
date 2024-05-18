
import { createMachine } from 'xstate';

export const Torrent = createMachine({
  id: 'torrent',
  initial: 'idle'
})

export type Torrent = typeof Torrent
