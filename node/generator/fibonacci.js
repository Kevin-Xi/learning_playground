function* fibonacci() {
    let fst = 1, snd = 1;
    while (true) {
        let cur = fst;
        fst = snd;
        snd += cur;
        let stop = yield cur;
        if (stop) break;
    }
}

let i = 0;
let fibStream = fibonacci();

while (i++ < 15) {
    console.log(fibStream.next(i === 10));
}
