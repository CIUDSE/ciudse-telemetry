export default function HelloWorldPlugin() {
    return function install() {
        console.log("Hello world!");
    };
}