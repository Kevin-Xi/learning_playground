import {go, chan, take, putAsync} from 'js-csp';

let ch = chan();

go(function* () {
    const received = yield take(ch);
    console.log(`received ${received}`);
});

const msg = 'msg';
console.log(`sending ${msg}`);
putAsync(ch, msg);
