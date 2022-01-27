import CiudseTelemetryPlugin from './plugins/ciudse-telemetry/ciudse-telemetry'
import CesiumMapPlugin from './plugins/cesium-map/cesium-map'

const ONE_SECOND = 1000
const THIRTY_SECONDS = 30 * ONE_SECOND
const ONE_MINUTE = 60 * ONE_SECOND
const TEN_MINUTES = 10 * ONE_MINUTE

openmct.setAssetPath('openmct')
openmct.install(openmct.plugins.LocalStorage())
openmct.install(openmct.plugins.MyItems())
openmct.install(openmct.plugins.Espresso())
openmct.install(openmct.plugins.UTCTimeSystem())
openmct.install(openmct.plugins.Conductor({
  menuOptions: [
    {
      name: 'Fixed',
      timeSystem: 'utc',
      bounds: {
        start: Date.now() - TEN_MINUTES,
        end: Date.now()
      }
    },
    {
      name: 'Realtime',
      timeSystem: 'utc',
      clock: 'local',
      clockOffsets: {
        start: -ONE_MINUTE,
        end: THIRTY_SECONDS
      }
    }
  ]
}))

const server_domain = window.location.hostname
const server_port = 8080
openmct.install(CiudseTelemetryPlugin(server_domain, server_port))

openmct.install(CesiumMapPlugin())

document.addEventListener('DOMContentLoaded', () => openmct.start(document.body))
