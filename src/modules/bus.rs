use crate::modules::cart::Cart;

pub struct Bus<'a> {
    cart: &'a mut Cart,
}

impl<'a> Bus<'a> {
    pub fn new(cart: &'a mut Cart) -> Self {
        Self { cart: cart }
    }

    pub fn read(&self, address: u16) -> u8 {
        if address < 0x8000 {
            return self.cart.read(address);
        }
        panic!("Bus read not implemented for address: {:X}", address);
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            self.cart.write(address, value);
        }
        panic!("Bus write not implemented")
    }
}
