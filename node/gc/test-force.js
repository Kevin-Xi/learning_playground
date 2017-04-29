/**
 * 1. test without gc: node test-force.js
 * 2. test with gc: node --expose-gc test-force.js
 * for 1, only if other heavy memory-used process push it, it will gc.
 * for 2, it will gc after call global.gc()
 */

'use strict';

function test() {
    console.info('before init');
    let bigObj = initBigObj();
    console.info('after init');

    setTimeout(function () {
        console.info('before null');
        bigObj = {};
        console.info('after null');

        if (global.gc) {
            console.info('before gc');
            global.gc();
            console.info('after gc');
        }
    }, 60000);
}

let i = 0;
setInterval(showMem, 2000);
setTimeout(test, 10000);

function showMem() {
    let {rss, heapTotal, heapUsed} = process.memoryUsage();
    console.info(`${i++}. rss: ${formatBytes(rss)}, heapTotal: ${formatBytes(heapTotal)}, heapUsed: ${formatBytes(heapUsed)}`);
}

function initBigObj() {
    let bigObj = {};
    for (let i = 0; i < 5e7; i++) {
        bigObj[i] = i * 2;
    }
    return bigObj;
}

function formatBytes(bytes,decimals) {
       if(bytes == 0) return '0 Bytes';
       var k = 1000,
               dm = decimals + 1 || 3,
               sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'],
               i = Math.floor(Math.log(bytes) / Math.log(k));
       return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}
