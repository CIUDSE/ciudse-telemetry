class HistoricalTelemetryProvider {
  supportsRequest (domain_object) {
    return domain_object.type === 'example.telemetry'
  }

  request (_domain_object, _options) {
    return new Promise((resolve) => {
      resolve({
        timestamp: Date.now(),
        value: 0
      })
    })
  }
}

export default function HistoricalTelemetryPlugin () {
  return function install (openmct) {
    openmct.telemetry.addProvider(new HistoricalTelemetryProvider())
  }
}
