const async_hooks = require('async_hooks');

async_hooks.createHook({
    init (id, type, triggerId) {
        const cId = async_hooks.currentId();
        process._rawDebug(`${type}(${id}): trigger: ${triggerId} scope: ${cId}`);
    }
}).enable();

require('net').createServer(c => {}).listen(8080);
