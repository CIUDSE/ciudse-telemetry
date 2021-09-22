import { getFullKey } from './utils'

class HistoricalTelemetryProvider {
  constructor (server_domain, server_port) {
    this.server_domain = server_domain
    this.server_port = server_port
  }

  supportsRequest (domain_object) {
    return domain_object.type === 'example.telemetry'
  }

  request (domain_object, options) {
    const key = getFullKey(domain_object)
    return fetch(
      `http://${this.server_domain}:${this.server_port}/historical/${key}?start=${options.start},end=${options.end}`
    ).then(response => response.json())
  }
}

export default function HistoricalTelemetryPlugin (server_domain, server_port) {
  return function install (openmct) {
    openmct.telemetry.addProvider(new HistoricalTelemetryProvider(server_domain, server_port))
  }
}
