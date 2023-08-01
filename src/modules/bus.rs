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
        panic!("Bus write not implemented for address: {:X}", address)
    }

    pub fn read16(&self, address: u16) -> u16 {
        let lo: u16 = self.read(address) as u16;
        let hi: u16 = self.read(address + 1) as u16;

        return lo | (hi << 8);
    }

    pub fn write16(&mut self, address: u16, data: u16) {
        self.write(address + 1, ((data >> 8) & 0xFF) as u8);
        self.write(address, (data & 0xFF) as u8);
    }
}
