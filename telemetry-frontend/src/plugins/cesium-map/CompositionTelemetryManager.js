export default class TelemetryObjectManager {
  constructor (parentDomainObject, onAddCallback, onRemoveCallback) {
    this.domainObject = parentDomainObject
    this.setOnAddCallback(onAddCallback)
    this.setOnRemoveCallback(onRemoveCallback)
    this.childDomainObjects = new Map()
    this.composition = openmct.composition.get(this.domainObject)
    if (this.composition !== undefined) {
      this.composition.load().then((comp) => {
        comp.forEach((domainObject) => { this.addTelemetryObject(domainObject) })
        this.composition.on('add', this.addTelemetryObject)
        this.composition.on('remove', this.removeTelemetryObject)
      })
    }
  }

  isTelemetryObject (domainObject) {
    return Object.prototype.hasOwnProperty.call(domainObject, 'telemetry')
  }

  setOnAddCallback (onAddCallback) {
    this.onAddCallback = onAddCallback
  }

  setOnRemoveCallback (onRemoveCallback) {
    this.onRemoveCallback = onRemoveCallback
  }

  getChildren () {
    return this.childDomainObjects
  }

  addTelemetryObject (domainObject) {
    console.log(domainObject)
    if (!this.isTelemetryObject(domainObject)) {
      return
    }
    if (this.childDomainObjects.has(domainObject.identifier)) {
      return
    }
    this.childDomainObjects.set(domainObject.identifier, domainObject)
    if (this.onAddCallback !== undefined) {
      this.onAddCallback(domainObject)
    }
  }

  removeTelemetryObject (domainObject) {
    if (!this.childDomainObjects.has(domainObject.identifier)) {
      return
    }
    this.childDomainObjects.delete(domainObject.identifier)
    if (this.onRemoveCallback !== undefined) {
      this.onRemoveCallback(domainObject)
    }
  }

  destroy () {
    this.composition.off('add', this.addTelemetryObject)
    this.composition.off('remove', this.removeTelemetryObject)
  }
}
