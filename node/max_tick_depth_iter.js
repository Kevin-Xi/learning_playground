process.maxTickDepth = 20;

setImmediate(function() {
  pre = process.hrtime(pre);
  console.log('setImmediate', pre[0] + pre[1] / 1e9);
});

function doop() {
  for (var i = 0; i < 1e3; i++)
  Math.atan(Math.random());
}

for (var j = 0; j < 1e5; j++)
process.nextTick(doop);

var pre = process.hrtime();