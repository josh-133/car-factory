mod car;
use car::{Car, Sedan, Suv, CarFactory, SedanFactory, SuvFactory};

fn main() {
    let sedan_factory = SedanFactory;
    let suv_factory = SuvFactory;

    let my_sedan = sedan_factory.create_car();
    let my_suv = suv_factory.create_car();

    my_sedan.drive();
    my_suv.drive();
}
