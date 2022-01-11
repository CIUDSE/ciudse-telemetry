import './style.css'

const domain_object_data = new Map()
require('./composition.json').forEach((domain_object) => {
  const namespace = domain_object.identifier.namespace
  if (!domain_object_data.has(namespace)) {
    domain_object_data.set(namespace, new Map())
  }
  const namespace_data = domain_object_data.get(namespace)
  namespace_data.set(domain_object.identifier.key, domain_object)
})

const type_data = require('./types.json')

class HistoricalTelemetryProvider {
  constructor (server_domain, server_port) {
    this.server_domain = server_domain
    this.server_port = server_port
  }

  supportsRequest (domain_object) {
    return domain_object_data.has(domain_object.identifier.namespace)
  }

  request (domain_object, options) {
    return fetch(
      `http://${this.server_domain}:${this.server_port}/historical?namespace=${domain_object.identifier.namespace}&key=${domain_object.identifier.key}&start=${Math.round(options.start)}&end=${Math.round(options.end)}`
    ).then(response => {
      return response.json()
    })
  }
}

class RealtimeTelemetryProvider {
  constructor (server_domain, server_port) {
    this.server_domain = server_domain
    this.server_port = server_port
  }

  supportsSubscribe (domain_object) {
    return domain_object_data.has(domain_object.identifier.namespace)
  }

  subscribe (domain_object, f) {
    const socket = new WebSocket(`ws://${this.server_domain}:${this.server_port}/realtime?namespace=${domain_object.identifier.namespace}&key=${domain_object.identifier.key}`)
    socket.onmessage = (event) => {
      const points = JSON.parse(event.data)
      points.forEach(point => {
        f(point)
      })
    }
    return function unsubscribe () {
      socket.close()
    }
  }
}

export default function CiudseTelemetryPlugin (server_domain, server_port) {
  return function install (openmct) {
    domain_object_data.forEach((namespace_data, namespace) => {
      openmct.objects.addProvider(namespace, {
        get: function (identifier) {
          const domain_object = namespace_data.get(identifier.key)
          const domain_object_type = domain_object.type
          const domain_object_type_data = type_data[domain_object_type]
          if (domain_object_type_data !== undefined) {
            const initializeValues = domain_object_type_data.initializeValues
            if (initializeValues !== undefined) {
              for (const k in initializeValues) {
                domain_object[k] = structuredClone(initializeValues[k])
              }
            }
          }
          return Promise.resolve(domain_object)
        }
      })
      namespace_data.forEach((domain_object) => {
        if (domain_object.location === 'ROOT') {
          openmct.objects.addRoot(domain_object.identifier)
        }
      })
    })
    for (const key in type_data) {
      openmct.types.addType(key, {
        name: type_data[key].name,
        description: type_data[key].description,
        cssClass: type_data[key].cssClass
      })
    }

    openmct.telemetry.addProvider(new HistoricalTelemetryProvider(server_domain, server_port))

    openmct.telemetry.addProvider(new RealtimeTelemetryProvider(server_domain, server_port))
  }
}
