use crate::model::Parameters;

pub trait Modelize<T> {
    fn modelize(self, parameters: &Parameters) -> T;
}

pub trait TryModelize<T> {
    type Error;

    fn try_modelize(self, parameters: &Parameters) -> Result<T, Self::Error>;
}

pub trait ModelizeWith<T> {
    type With;

    fn modelize_with(self, parameters: &Parameters, with: &Self::With) -> T;
}

pub trait TryModelizeWith<T> {
    type With;
    type Error;

    fn try_modelize_with(
        self,
        parameters: &Parameters,
        with: &Self::With,
    ) -> Result<T, Self::Error>;
}
