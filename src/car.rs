pub trait Car {
    fn drive(&self);
}

pub struct Sedan;
pub struct Suv;
pub struct Fwd;

impl Car for Sedan {
    fn drive(&self) {
        println!("Driving a Sedan!");
    }
}

impl Car for Suv {
    fn drive(&self) {
        println!("Driving a SUV!");
    }
}

impl Car for Fwd {
    fn drive(&self) {
        println!("Driving a Fwd!");
    }
}

pub trait CarFactory {
    fn create_car(&self) -> Box<dyn Car>;
}

pub struct SedanFactory;
pub struct SuvFactory;
pub struct FwdFactory;

impl CarFactory for SedanFactory {
    fn create_car(&self) -> Box<dyn Car> {
        Box::new(Sedan)
    }
}

impl CarFactory for SuvFactory {
    fn create_car(&self) -> Box<dyn Car> {
        Box::new(Suv)
    }
}

impl CarFactory for FwdFactory {
    fn create_car(&self) -> Box<dyn Car> {
        Box::new(Fwd)
    }
}