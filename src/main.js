import openmct from 'node_modules/openmct/dist/openmct'

const ONE_SECOND = 1000
const THIRTY_SECONDS = 30 * ONE_SECOND
const ONE_MINUTE = 60 * ONE_SECOND
const THIRTY_MINUTES = 30 * ONE_MINUTE

openmct.setAssetPath('openmct-assets')
openmct.install(new openmct.plugins.LocalStorage())
openmct.install(new openmct.plugins.Espresso())
openmct.install(new openmct.plugins.MyItems())
openmct.install(openmct.plugins.UTCTimeSystem())
openmct.install(openmct.plugins.Conductor({
  menuOptions: [
    {
      name: 'Fixed',
      timeSystem: 'utc',
      bounds: {
        start: Date.now() - THIRTY_MINUTES,
        end: Date.now()
      }
    },
    {
      name: 'Realtime',
      timeSystem: 'utc',
      clock: 'local',
      clockOffsets: {
        start: -THIRTY_MINUTES,
        end: THIRTY_SECONDS
      }
    }
  ]
}))
document.addEventListener('DOMContentLoaded', () => openmct.start(document.body))
