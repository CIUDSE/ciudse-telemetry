import {fullKey, getFullKey} from "./utils";

var spaceship_data = new Map();
require("./test-spaceship.json").forEach(domain_object => {
    const full_key = getFullKey(domain_object);
    spaceship_data.set(full_key, domain_object);
});

class DomainObjectProvider{
    get(identifier) {
        const full_key = fullKey(identifier);
        return spaceship_data.get(full_key);
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