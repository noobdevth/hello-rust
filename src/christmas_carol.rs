// Merry Christmas! :)

// Listen to Twelve Days of Christmas on Spotify:
// https://open.spotify.com/track/0DUcEfCnzRaoDKgumEsOh1
// https://open.spotify.com/track/604DTaRINInOPt26OBBNU6

const GIFTS: [&'static str; 12] = [
  "a Partridge in a Pear Tree",
  "Two Turtle Doves",
  "Three French Hens",
  "Four Calling Birds",
  "Five Golden Rings",
  "Six Geese-a-Laying",
  "Seven Swans a-Swimming",
  "Eight Maids a-Milking",
  "Nine Ladies Dancing",
  "Ten Lords-a-Leaping",
  "Eleven Pipers Piping",
  "Twelve Drummers Drumming",
];

const DAYS: [&'static str; 12] = [
  "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth",
  "Eleventh", "Twelveth",
];

fn get_gifts(day: usize) -> String {
  let gift = String::from(GIFTS[day - 1]);

  if day == 1 {
    // If this is the first day
    return gift;
  }

  let conjunction = if day == 2 { "and " } else { "" };

  String::from(gift + "\n" + conjunction + &get_gifts(day - 1))
}

fn main() {
  for day in 1..13 {
    println!(
      "On the {} day of Christmas my true love sent to me",
      DAYS[day - 1]
    );
    println!("{}\n", get_gifts(day as usize));
  }
}
