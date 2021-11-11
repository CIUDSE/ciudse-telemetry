import { fullKey, getFullKey } from './utils'

const spaceship_data = new Map()
require('./test-spaceship.json').forEach((domain_object) => {
  const full_key = getFullKey(domain_object)
  spaceship_data.set(full_key, domain_object)
})

export default function TestSpaceshipPlugin () {
  return function install (openmct) {
    console.log('Test spaceship enabled')
    openmct.objects.addRoot({
      namespace: 'test-spaceship',
      key: 'root'
    })
    openmct.objects.addRoot({
      namespace: 'kerbal',
      key: 'root'
    })
    openmct.objects.addProvider('test-spaceship', {
      get: function (identifier) {
        return Promise.resolve(spaceship_data.get(fullKey(identifier)))
      }
    })
    openmct.objects.addProvider('kerbal', {
      get: function (identifier) {
        return Promise.resolve(spaceship_data.get(fullKey(identifier)))
      }
    })
    openmct.types.addType('example.telemetry', {
      name: 'Example Telemetry Point',
      description: 'Example telemetry point from our happy tutorial.',
      cssClass: 'icon-telemetry'
    })
  }
}
