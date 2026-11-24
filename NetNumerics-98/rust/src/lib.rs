//! Numerical kernels shared with the browser through a small WebAssembly ABI.
fn trap(y: &[f64], h: f64) -> f64 { y.windows(2).map(|w| h*(w[0]+w[1])/2.0).sum() }
fn simp(y: &[f64], h: f64) -> f64 {
    if y.len()<2 {return 0.0;}
    let n=y.len()-1; let m=n-n%2;
    let mut s=0.0;
    if m>=2 { s=h/3.0*(y[0]+y[m]+(1..m).map(|i|y[i]*if i%2==1 {4.0}else{2.0}).sum::<f64>()); }
    if m<n {s+=h*(y[m]+y[m+1])/2.0;}
    s
}
#[no_mangle] pub extern "C" fn alloc(n: usize) -> *mut f64 {
    let b=vec![0.0_f64;n].into_boxed_slice(); Box::into_raw(b) as *mut f64
}
/// Safety: p must come from alloc(n), and must be freed exactly once.
#[no_mangle] pub unsafe extern "C" fn dealloc(p:*mut f64,n:usize){
    if !p.is_null(){drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(p,n)));}
}
/// Safety: p must point to at least n initialized f64 values.
#[no_mangle] pub unsafe extern "C" fn trapezoid(p:*const f64,n:usize,h:f64)->f64{
    if n<2||p.is_null(){return 0.0;} trap(std::slice::from_raw_parts(p,n),h)
}
/// Safety: p must point to at least n initialized f64 values.
#[no_mangle] pub unsafe extern "C" fn simpson(p:*const f64,n:usize,h:f64)->f64{
    if n<2||p.is_null(){return 0.0;} simp(std::slice::from_raw_parts(p,n),h)
}
#[no_mangle] pub extern "C" fn richardson(d_h:f64,d_2h:f64)->f64{(4.0*d_h-d_2h)/3.0}
#[cfg(test)] mod tests {use super::*;
 #[test] fn integration(){assert!((trap(&[0.,1.,2.],1.)-2.).abs()<1e-12);assert!((simp(&[0.,1.,4.],1.)-8./3.).abs()<1e-12);}
 #[test] fn derivative(){assert!((richardson(5.,8.)-4.).abs()<1e-12);}
}
