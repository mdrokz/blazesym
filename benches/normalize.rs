use blazesym::c_api;
use blazesym::normalize::normalize_user_addrs;
use blazesym::Addr;

use criterion::measurement::Measurement;
use criterion::BenchmarkGroup;


/// Normalize addresses in the current process.
fn normalize_process() {
    let mut addrs = [
        libc::__errno_location as Addr,
        libc::dlopen as Addr,
        libc::fopen as Addr,
        normalize_process as Addr,
        c_api::blazesym_find_addresses as Addr,
    ];
    let () = addrs.sort();

    let norm_addrs = normalize_user_addrs(addrs.as_slice(), 0).unwrap();
    assert_eq!(norm_addrs.addrs.len(), 5);
}

pub fn benchmark<M>(group: &mut BenchmarkGroup<'_, M>)
where
    M: Measurement,
{
    group.bench_function(stringify!(normalize::normalize_process), |b| {
        b.iter(normalize_process)
    });
}