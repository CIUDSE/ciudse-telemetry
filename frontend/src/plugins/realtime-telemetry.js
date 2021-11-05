import { getFullKey } from './utils'

class RealtimeTelemetryProvider {
  constructor (server_domain, server_port) {
    this.server_domain = server_domain
    this.server_port = server_port
  }

  supportsSubscribe (domain_object) {
    return domain_object.type === 'example.telemetry'
  }

  subscribe (domain_object, f) {
    const key = getFullKey(domain_object)
    const socket = new WebSocket(`ws://${this.server_domain}:${this.server_port}/realtime/${key}`)
    socket.onmessage = (event) => {
      const point = JSON.parse(event.data)
      f(point)
    }
    return function unsubscribe () {
      socket.close()
    }
  }
}

export default function RealtimeTelemetryPlugin (server_domain, server_port) {
  return function install (openmct) {
    openmct.telemetry.addProvider(new RealtimeTelemetryProvider(server_domain, server_port))
  }
}
