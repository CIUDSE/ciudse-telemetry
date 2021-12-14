import './style.css'

Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI2MjMyNjkxYi05M2UyLTQzMjgtYmNlMy1lYmFjYWJiN2UwM2IiLCJpZCI6NDUyMjUsImlhdCI6MTYxNDgzMTQ5NX0.zcDxbuMRtO_FucfP6IcjCWI7SNFvVR3vlZ37BGL7GBM'

class CesiumMapView {
  constructor (_domainObject, _openmct) {
    console.debug('Map View created!')
  }

  show (element) {
    console.debug('CesiumMapPlugin show')
    if (element.id === '') {
      element.id = 'cesium-container'
    }
    this.viewer = new Cesium.Viewer(element.id, {
      terrainProvider: Cesium.createWorldTerrain()
    })
  }

  destroy () {
    console.debug('CesiumMapPlugin destroy')
  }
}

export default function CesiumMapPlugin () {
  return function install (openmct) {
    openmct.objectViews.addProvider({
      key: 'cesium',
      name: 'Cesium Map',
      cssClass: 'icon-cesium',
      canView: (domain_object) => {
        return domain_object.type === 'example.telemetry'
      },
      view: (_domain_object) => {
        return new CesiumMapView()
      }
    })
  }
}
