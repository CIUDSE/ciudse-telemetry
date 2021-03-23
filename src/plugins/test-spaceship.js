class DomainObjectProvider{
    constructor(){
        this.cache = new Map();
        this.is_cached = false;
    }

    getDomainObjects() {
        if(this.is_cached){
            // eslint-disable-next-line no-unused-vars
            return new Promise((resolve, reject) => {
                resolve(this.cache);
            });
        }
        return fetch("test-spaceship.json")
            .then(response => response.text())
            .then(value => {
                const objects = JSON.parse(value);
                objects.forEach(object => {
                    const full_name = object.identifier.namespace + "." + object.identifier.key;
                    this.cache.set(full_name, object);
                });
                this.is_cached = true;
                return this.cache;
            });
    }
    
    get(identifier) {
        const full_name = identifier.namespace + "." + identifier.key;
        return this.getDomainObjects().then(object_map => {
            return object_map.get(full_name);
        });
    }
}

export default function TestSpaceshipPlugin(){
    return function install(openmct) {
        console.log("Test spaceship enabled");
        openmct.objects.addRoot({
            namespace: "test-spaceship",
            key: "root"
        });
        openmct.objects.addProvider("test-spaceship", new DomainObjectProvider());

        openmct.types.addType("example.telemetry", {
            name: "Example Telemetry Point",
            description: "Example telemetry point from our happy tutorial.",
            cssClass: "icon-telemetry"
        });
    };
}