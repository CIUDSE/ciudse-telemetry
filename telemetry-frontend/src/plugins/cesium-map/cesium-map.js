import './style.css'

Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI2MjMyNjkxYi05M2UyLTQzMjgtYmNlMy1lYmFjYWJiN2UwM2IiLCJpZCI6NDUyMjUsImlhdCI6MTYxNDgzMTQ5NX0.zcDxbuMRtO_FucfP6IcjCWI7SNFvVR3vlZ37BGL7GBM'

class CesiumMapView {
  constructor (domainObject, openmct) {
    console.debug('Map View created!')
    this.domainObject = domainObject
    this.openmct = openmct
    // this.unlistenFromMutation = openmct.objects.observe(domainObject
  }

  show (element) {
    console.debug('CesiumMapPlugin show')
    this.viewer = new Cesium.Viewer(element, {
      animation: false,
      baseLayerPick: false,
      fullscreenButton: false,
      vrButton: false,
      geocoder: false,
      homeButton: false,
      infoBox: false,
      sceneModePicker: true,
      selectionIndicator: false,
      timeline: false,
      navigationHelpButton: false,
      navigationInstructionsInitiallyVisible: false,
      scene3DOnly: false,
      shouldAnimate: false,
      shadows: false
    })

    this.telemetryCollection = this.openmct.telemetry.requestCollection(this.domainObject)
    this.telemetryCollection.load()
    this.cartesianPositions = []

    this.intervalId = setInterval(() => {
      this.cartesianPositions = this.telemetryCollection.getAll().map((datum) => {
        return Cesium.Cartesian3.fromRadians(datum.lng, datum.lat, datum.hgt)
      })
    }, 200)

    const lineEntity = this.viewer.entities.add({
      name: this.domainObject.name,
      polyline: {
        positions: new Cesium.CallbackProperty(() => {
          return this.cartesianPositions
        }),
        width: 5,
        material: new Cesium.PolylineOutlineMaterialProperty({
          color: Cesium.Color.ORANGE,
          outlineWidth: 2,
          outlineColor: Cesium.Color.BLACK
        })
      }
    })

    this.viewer.zoomTo(lineEntity)
  }

  destroy () {
    clearInterval(this.intervalId)
  }
}

export default function CesiumMapPlugin () {
  return function install (openmct) {
    openmct.types.addType('ciudse.types.view.cesium', {
      name: 'Cesium Map',
      description: 'Cesium Map',
      creatable: true,
      cssClass: 'icon-ciudse-types-view-cesium'
    })
    openmct.objectViews.addProvider({
      key: 'cesium',
      name: 'Cesium Map',
      cssClass: 'icon-ciudse-types-view-cesium',
      canView: (domain_object) => {
        return domain_object.type === 'ciudse.types.view.cesium' || domain_object.type === 'cesium.geodatum'
      },
      view: (domain_object) => {
        return new CesiumMapView(domain_object, openmct)
      }
    })
  }
}
