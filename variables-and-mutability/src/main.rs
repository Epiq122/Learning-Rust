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
}
