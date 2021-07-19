use libfive::{BRepSettings, Region3, Tree, TreeVec3};

fn main() {
    let outer_diameter = 30.0;
    let height = 10.0;

    // TASK: Model actual spacer.
    let spacer = Tree::cylinder_z(
        (outer_diameter / 2.0).into(),
        height.into(),
        TreeVec3::new(0.0, 0.0, 0.0),
    );

    let xy_max = outer_diameter * 1.1;
    let z_min = -height * 0.1;
    let z_max = height * 1.1;

    spacer
        .to_stl(
            "spacer.stl",
            &Region3::new(-xy_max, xy_max, -xy_max, xy_max, z_min, z_max),
            &BRepSettings::default(),
        )
        // TASK: Improve error handling.
        .unwrap();
}
