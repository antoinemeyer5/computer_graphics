use linrust::linvec2::Linvec2;

/*********************************************************** INTEGRATION TEST */

#[test]
fn linvec2_pipeline_works() {
    let v = Linvec2 { x: 1.0, y: 2.0 }
        .add(Linvec2 { x: 3.0, y: 4.0 })
        .scale(2.0);

    assert!(v.x == 8.0);
    assert!(v.y == 12.0);
}
