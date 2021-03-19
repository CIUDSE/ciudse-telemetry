export default function TestSpaceshipPlugin(){
    return function install(openmct) {
        console.log('Test spaceship enabled');
        openmct.objects.addRoot({
            namespace: 'test-spaceship',
            key: 'my-key'
        });
        openmct.objects.addProvider('test-spaceship',{
            get: function(identifier){
                if (!(identifier.namespace === 'test-spaceship')) { return }
                var o = {};
                switch (identifier.key) {
                    case 'my-key':
                        o = {
                            identifier: identifier,
                            name: 'Test spaceship',
                            type: 'folder',
                            composition: [
                                {
                                    key: 'fuel',
                                    namespace: 'test-spaceship'
                                }
                            ]
                        }
                        break;
                    case 'fuel':
                        o = {
                            identifier: identifier,
                            name: 'Fuel',
                            type: 'folder'
                        }
                    default:
                        break;
                }
                return Promise.resolve(o)
            }
        })
    }
};