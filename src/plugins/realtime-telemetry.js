import { getFullKey } from './utils'

class RealtimeTelemetryProvider {
  constructor (url) {
    this.url = url
  }

  supportsSubscribe (domain_object) {
    return domain_object.type === 'example.telemetry'
  }

  subscribe (domain_object, f) {
    const socket = new WebSocket(this.url + getFullKey(domain_object))
    socket.onmessage = (event) => {
      const point = JSON.parse(event.data)
      f(point)
    }
    return function unsubscribe () {
      socket.close()
    }
  }
}

export default function RealtimeTelemetryPlugin (uri) {
  return function install (openmct) {
    openmct.telemetry.addProvider(new RealtimeTelemetryProvider(uri))
  }
}
