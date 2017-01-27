// https://cnodejs.org/topic/519b523c63e9f8a5429b25e3
// http://stackoverflow.com/questions/15349733/setimmediate-vs-nexttick
// https://nodejs.org/en/blog/release/v0.10.0/
// https://nodejs.org/dist/latest-v7.x/docs/api/timers.html#timers_setimmediate_callback_args
// https://nodejs.org/dist/latest-v7.x/docs/api/process.html#process_process_nexttick_callback_args
//
'use strict';
const fs = require('fs');
const TOTAL = 20;

setImmediate(() => {
    console.info(`setImmediate 0 run`);
});
process.nextTick(() => {
    console.info(`nextTick 0 run`);
});
process.nextTick(console.info, `nextTick 0.5 run`);
setImmediate(console.info, `setImmediate 0.5 run`);
process.nextTick(() => {
    console.info(`nextTick 1 run`);
});
process.nextTick(() => {
    console.info(`nextTick 2 run`);
});
process.nextTick(() => {
    console.info(`nextTick 3 run`);
});
process.nextTick(() => {
    console.info(`nextTick with before run`);
    fs.access(`.`, (err, result) => {
        console.info(`async task before come back`);
    });
});
setImmediate(() => {
    console.info(`setImmediate 1 run`);
});
console.info(`async nextTick 0 start`);
process.nextTick(fs.access, `.`, () => {console.log(`async nextTick 0 come back`)});
console.info(`async start`);
fs.access(`.`, (err, result) => {
    console.info(`async task come back`);
});
console.info(`async nextTick 1 start`);
process.nextTick(fs.access, `.`, () => {console.log(`async nextTick 1 come back`)});
process.nextTick(() => {
    console.info(`nextTick with after run`);
    fs.access(`.`, (err, result) => {
        console.info(`async task after come back`);
    });
});
process.nextTick(() => {
    console.info(`nextTick 4 run`);
});
process.nextTick(() => {
    console.info(`nextTick 5 run`);
});
process.nextTick(() => {
    console.info(`nextTick 6 run`);
});
setImmediate(() => {
    console.info(`setImmediate 2 run`);
});
process.nextTick(() => {
    console.info(`nextTick 7 run`);
});
