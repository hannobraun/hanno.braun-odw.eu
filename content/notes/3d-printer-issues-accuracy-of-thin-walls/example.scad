diameter = 20.0;
height   = 10.0;
wall     =  0.6;

$fn = 360;

difference() {
    outer = diameter / 2;
    inner = outer - wall;

    cylinder(r = outer, h = height);
    cylinder(r = inner, h = height);
}
