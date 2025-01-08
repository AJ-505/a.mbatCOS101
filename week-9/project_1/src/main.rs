struct Electronics {
    hp:u32, ibm:u32, toshiba:u32, dell:u32
}
impl Electronics {
    fn calculate_cost(&self) -> u32 {
        println!("The package that was received was 10 HP, 6 IBM, 10 Toshiba and 4 Dell Laptops.");
        println!("This customer ordered:\n{} HP, {} IBM, {} Toshiba and {} Dell Laptops.", self.hp, self.ibm, self.toshiba, self.dell);
        self.hp * 650_000 + self.ibm * 755_000 + self.toshiba * 550_000 + self.dell * 850_000
    }
}
fn main() {
    let customer1 = Electronics {
        hp:3, ibm:3, toshiba:3, dell:3
    };
    println!("The cost of this customer's purchase is {}.", customer1.calculate_cost());
    println!("Thank you for using this program!");
}
