'use strict';

const Status = {
    Pending: 1,
    Resloved: 2
};

module.exports = class HomemadePromise {
    constructor() {
        this.cont = null;
        this.status = Status.Pending;
        this.next = null;
    }

    then(cont) {
        this.cont = cont;
        let p = new HomemadePromise();
        this.next = p;
        return p;
    }

    resolve(res) {
        if (this.status === Status.Resloved) throw new Error('had resloved');
        this.status = Status.Resloved;
        if (!this.cont) return;
        let p = this.cont(res);
        if (p && p.then) {
            p.then(res_cont => this.next.resolve(res_cont));
        } else {
            this.next.resolve(p);
        }
    }

    reject(err) {
        console.log(err);
    }
}
