struct HeroPlayer {
    username: String,
    email: String,
    level: u64,
    hp: u64,
    strength: u64,
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = build_hero_player(String::from("test@gmail.com"), String::from("Matt"));

    println!("Hero created: {}", user1.username);

    println!("{} level: {}", user1.username, user1.level);

    user1.level = 2;

    println!("{} level: {}", user1.username, user1.level);

    let user2 = HeroPlayer {
        username: String::from("Tadd"),
        email: String::from("tadd@gmail.com"),
        ..user1
    };

    println!("Hero created: {}", user2.username);

    println!("{} level: {}", user1.username, user1.level);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_hero_player(email: String, username: String) -> HeroPlayer {
    HeroPlayer {
        username,
        email,
        level: 1,
        hp: 100,
        strength: 10,
    }
}
