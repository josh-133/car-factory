mod car;
use car::{CarFactory, SedanFactory, SuvFactory, FwdFactory};

fn main() {
    let sedan_factory = SedanFactory;
    let suv_factory = SuvFactory;
    let fwd_factory = FwdFactory;


    let my_sedan = sedan_factory.create_car();
    let my_suv = suv_factory.create_car();
    let my_fwd = fwd_factory.create_car();

    my_sedan.drive();
    my_suv.drive();
    my_fwd.drive();
}
