import {fullKey, getFullKey} from "./utils";

class DomainObjectProvider{
    constructor(){
        this.cache = new Map();
        this.is_cached = false;
    }

    getDomainObjects() {
        if(this.is_cached){
            return new Promise((resolve, _reject) => {
                resolve(this.cache);
            });
        }
        return fetch("test-spaceship.json")
            .then(response => response.text())
            .then(value => {
                const objects = JSON.parse(value);
                objects.forEach(domain_object => {
                    const full_key = getFullKey(domain_object);
                    this.cache.set(full_key, domain_object);
                });
                this.is_cached = true;
                return this.cache;
            });
    }
    
    get(identifier) {
        const full_key = fullKey(identifier);
        return this.getDomainObjects().then(object_map => {
            return object_map.get(full_key);
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