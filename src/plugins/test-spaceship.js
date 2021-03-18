export default function TestSpaceshipPlugin(){
    return function install(openmct) {
        console.log('Test spaceship enabled');
        openmct.objects.addRoot({
            namespace: 'example.taxonomy',
            key: 'test-spaceship'
        });
    }
};