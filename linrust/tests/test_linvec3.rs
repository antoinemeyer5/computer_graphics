use linrust::linvec3::Linvec3;

/*********************************************************** INTEGRATION TEST */

#[test]
fn linvec3_pipeline_works() {
    let v = Linvec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    }
    .add(Linvec3 {
        x: 3.0,
        y: 4.0,
        z: 5.0,
    })
    .cross(Linvec3 {
        x: 6.0,
        y: 7.0,
        z: 8.0,
    });

    assert!(v.x == -8.0);
    assert!(v.y == 16.0);
    assert!(v.z == -8.0);
}
