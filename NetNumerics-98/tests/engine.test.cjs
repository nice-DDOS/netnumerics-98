const assert=require('node:assert/strict'),N=require('../engine.js');const near=(x,y,t=1e-8)=>assert.ok(Math.abs(x-y)<t,`${x} != ${y}`);
near(N.gaussian([[2,1],[1,-1]],[5,1]).x[0],2);near(N.fit([2,5,8,11,14]).coef[1],3);near(N.fit([1,4,9,16,25],2).coef[2],1);
for(const m of ['Bisection','Newton','Secant'])near(N.rootSolve(x=>x*x-2,x=>2*x,1,2,m).root,Math.SQRT2,1e-7);
assert.equal(N.rootSolve(x=>x*x+1,x=>2*x,-1,1,'Bisection').root,null);near(N.lagrange([0,1,3],[0,1,9],2),4);near(N.divided([0,1,3],[0,1,9],2),4);near(N.integrate([0,1,4],1).simpson,8/3);near(N.integrate([0,1,2,3],1).simpson,4.5);
let a=N.analyze(N.demo('surge'),20,18);assert.ok(a.alerts.length>0&&a.cross!==null);assert.ok(a.roots.every(r=>r.root!==null));assert.equal(N.analyze(N.demo('normal'),20,18).alerts.length,0);assert.ok(N.analyze(N.demo('spike'),20,18).alerts.length>0);
let csv='time_s,events_per_s\n'+N.demo('normal').map(v=>`${v.t},${v.y}`).join('\n');assert.equal(N.parseCSV(csv).length,61);assert.throws(()=>N.parseCSV(csv.replace('1,','-1,')));assert.throws(()=>N.analyze(N.demo('normal'),2,18));
console.log('PASS: Gaussian, linear/quadratic fit, 3 root solvers, interpolation, quadrature, 3 detection scenarios, CSV validation, parameter validation.');console.log('Surge:',{alerts:a.alerts.length,cross:a.cross,rmse:a.model.rmse,excess:a.int.trap});
