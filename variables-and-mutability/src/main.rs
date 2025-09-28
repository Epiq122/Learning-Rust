// Constant
const TAX_RATE: f64 = 7.25;

// Type alias
type Meters = i32;

fn main() {
    //  Variables

    let dvd = 50;
    let vhs = 11;
    let _media_owned = dvd + vhs;

    //     println!("The total amount of media i am selling is {media_owned}.",)
    println!("This year i bought {0} dvds and {1} vhs tapes", dvd, vhs);

    //    Immutable and Mutable variables
    let gym_reps = 10;
    println!("I plan to do {gym_reps} reps");

    //     gym_reps = 15; this wont work

    let mut buy_movie = 11;
    println!("I plan on buying {buy_movie} movies today");
    buy_movie = 7;
    println!("Decided to get {buy_movie} more movies today");

    // variable shadowing

    let _grams_of_protein = "100.345";
    let _grams_of_protein = 100.345;
    let _grams_of_proten = 100;

    // Scope

    //  outer
    let coffee_price = 5.99;

    {
        // inner
        println!("The price is {coffee_price}");
        let cookie_price = 1.99;
        println!("The cookie price is {cookie_price}");
        let coffee_price = 3.99;
        println!("The price is {coffee_price}");
    }

    // Constant
    let income = 10000;
    println!("The tax rate is {TAX_RATE}");
    println!("My income in {income} and my tax rate is {TAX_RATE}");

    // Type alias

    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!(
        "a mile long race is {mile_race_length} meters long and a two mile race is {two_mile_race_length}"
    );
}
