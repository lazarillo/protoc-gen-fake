use fake::{Dummy, Fake, Faker};
use fake::faker::name::en::Name; // Example for English names
use fake::faker::internet::en::SafeEmail; // Example for English safe emails

#[derive(Debug, Dummy)]
#[allow(dead_code)]
struct User {
    #[dummy(faker = "Name()")]
    name: String,
    #[dummy(faker = "SafeEmail()")]
    email: String,
    #[dummy(faker = "18..65")] // Example: age between 18 and 65
    age: u8,
}

fn main() {
    // Generate a single fake user
    let user: User = Faker.fake();
    println!("Generated user: {:?}", user);

    // Generate multiple fake users
    let users: Vec<User> = (0..5).map(|_| Faker.fake()).collect();
    println!("Generated users: {:?}", users);

    // Generate a simple fake string directly
    let random_word: String = fake::faker::lorem::en::Word().fake();
    println!("Random word: {}", random_word);
}