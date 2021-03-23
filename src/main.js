import HelloWorldPlugin from "./plugins/hello-world";
import TestSpaceshipPlugin from "./plugins/test-spaceship";
import HistoricalTelemetryPlugin from "./plugins/historical-telemetry";
import RealtimeTelemetryPlugin from "./plugins/realtime-telemetry";

const ONE_SECOND = 1000;
const THIRTY_SECONDS = 30 * ONE_SECOND;
const ONE_MINUTE = 60 * ONE_SECOND;
const THIRTY_MINUTES = 30 * ONE_MINUTE;

openmct.setAssetPath("openmct");
openmct.install(openmct.plugins.LocalStorage());
openmct.install(openmct.plugins.MyItems());
openmct.install(openmct.plugins.Espresso());
openmct.install(openmct.plugins.UTCTimeSystem());
openmct.install(openmct.plugins.Conductor({
    menuOptions: [
        {
            name: "Fixed",
            timeSystem: "utc",
            bounds: {
                start: Date.now() - THIRTY_MINUTES,
                end: Date.now()
            }
        },
        {
            name: "Realtime",
            timeSystem: "utc",
            clock: "local",
            clockOffsets: {
                start: -THIRTY_MINUTES,
                end: THIRTY_SECONDS
            }
        }
    ]
}));

openmct.install(HelloWorldPlugin());
openmct.install(TestSpaceshipPlugin());
openmct.install(HistoricalTelemetryPlugin());
openmct.install(RealtimeTelemetryPlugin("ws://localhost:8081/ws/"));

document.addEventListener("DOMContentLoaded", () => openmct.start(document.body));
