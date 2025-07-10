
use super::*;

#[test]
fn test_mode_strategy() {

    let o = FStrategy::Open;
    let c = FStrategy::Create;
    let oc = FStrategy::OpenOrCreate;

    assert_eq!(o.flags(), [false, false, false, false]);
    assert_eq!(c.flags(), [false, false, false, true]);
    assert_eq!(oc.flags(), [false, false, true, false]);

}

#[test]
fn test_mode_access() {

    let r = FAccess::Read;
    let w = FAccess::Write;
    let wr: FAccess = FAccess::ReadWrite;

    assert_eq!(r.flags(), [true, false, false, false]);
    assert_eq!(w.flags(), [false, true, false, false]);
    assert_eq!(wr.flags(), [true, true, false, false]);
    
}

#[test]
fn test_mode_combine_flags() {
    
    fn int_to_flags(n: u8) -> [bool; 4] {
        [
            (n >> 0) & 1 != 0,
            (n >> 1) & 1 != 0,
            (n >> 2) & 1 != 0,
            (n >> 3) & 1 != 0,
        ]
    }

    fn combine_bools(a: &[bool; 4], b: &[bool; 4]) -> [bool; 4] {
        [
            a[0] || b[0],
            a[1] || b[1],
            a[2] || b[2],
            a[3] || b[3],
        ]
    }   
    for a in 0u8..16 {
        for b in 0u8..16 {
            let fa = int_to_flags(a);
            let fb = int_to_flags(b);
            let expected = combine_bools(&fa, &fb);
            let actual = FMode::combine_flags(fa, fb);
            assert_eq!(actual, expected);
        }
    }
    
    let st = vec![
        FStrategy::Open,
        FStrategy::Create,
        FStrategy::OpenOrCreate
    ];

    let ac = vec![
        FAccess::Read,
        FAccess::Write,
        FAccess::ReadWrite
    ];

    for strategy in st.iter() {
        for access in ac.iter() {

            let strategy = strategy.flags();
            let access = access.flags();

            assert_eq!(FMode::combine_flags(strategy, access), combine_bools(&strategy, &access));
        }
    }

}

#[test]
fn test_mode_flags(){

    fn combine_bools(a: &[bool; 4], b: &[bool; 4]) -> [bool; 4] {
        [
            a[0] || b[0],
            a[1] || b[1],
            a[2] || b[2],
            a[3] || b[3],
        ]
    }  

    let st = vec![
        FStrategy::Open,
        FStrategy::Create,
        FStrategy::OpenOrCreate
    ];

    let ac = vec![
        FAccess::Read,
        FAccess::Write,
        FAccess::ReadWrite
    ];

    for strategy in st.iter() {
        for access in ac.iter() {
           let fm = FMode(*strategy, *access); 
           let c = combine_bools(&strategy.flags(), &access.flags());
           let c = (c[0], c[1], c[2], c[3]);
           assert_eq!(fm.flags(), c);
        }
    }   

}


#[test]
fn test_mode_get_handle() {

    FBin::remove_file("test_mode_get_handle").ok();
    let f = FMode(FStrategy::Open, FAccess::Read).get_handle("test_mode_get_handle");
    assert!(f.is_err());
    
    FBin::remove_file("test_mode_get_handle").ok();
    let f = FMode(FStrategy::Open, FAccess::Write).get_handle("test_mode_get_handle");
    assert!(f.is_err());

    FBin::remove_file("test_mode_get_handle").ok();
    let f = FMode(FStrategy::Open, FAccess::ReadWrite).get_handle("test_mode_get_handle");
    assert!(f.is_err());

    // FBin::remove_file("test_mode_get_handle").ok();
    // let f = FMode(FStrategy::Create, FAccess::Read).get_handle("test_mode_get_handle");
    // assert!(f.is_ok());
}









