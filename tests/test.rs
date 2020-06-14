use configparser::ini::Ini;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let mut config = Ini::new();
	let map = config.load("test.ini")?;
	assert_eq!(config.get("DEFAULT", "defaultvalues").unwrap(), "defaultvalues");
	assert_eq!(config.get("topsecret", "KFC").unwrap(), "the secret herb is orega-");
	assert_eq!(config.get("topsecret", "Empty string").unwrap(), "");
	assert_eq!(config.get("topsecret", "None string"), None);
	assert_eq!(config.get("spacing", "indented").unwrap(), "indented");
	assert_eq!(config.get("spacing", "not indented").unwrap(), "not indented");
	assert_eq!(map["DEFAULT"]["defaultvalues"].unwrap(), "defaultvalues");
	assert_eq!(map["topsecret"]["KFC"].unwrap(), "the secret herb is orega-");
	assert_eq!(map["topsecret"]["Empty string"].unwrap(), "");
	assert_eq!(map["topsecret"]["None string"], None);
	assert_eq!(map["spacing"]["indented"].unwrap(), "indented");
	assert_eq!(map["spacing"]["not indented"].unwrap(), "not indented");
	let mut_map = config.get_mut_map();
	mut_map.get_mut("topsecret").unwrap().insert(String::from("None string"), Some(String::from("None string")));
	assert_eq!(map["topsecret"]["None string"].clone().unwrap(), "None string");
	mut_map.clear();
	assert!(config.get_map_ref().is_empty());
	match doc() {
		Err(why) => panic!("{}", "Doc failed!"),
		Ok(_) => println!("{}", "Doc passed!")
	}
	Ok(())
}

fn doc() -> Result<(), Box<dyn Error>> {
  let mut config = Ini::new();

  // You can easily load a file to get a clone of the map:
  let map = config.load("../tests/test.ini")?;
  println!("{:?}", map);
  // You can also safely not store the reference and access it later with get_map_ref() or get a clone with get_map()

  // You can then access it like a normal hashmap:
  let innermap = map["topsecret"].clone(); // Remember this is a hashmap!

  // If you want to access the value, then you can simply do:
  let val = map["topsecret"]["KFC"].clone().unwrap();
  // Remember that the .clone().unwrap() is required because it's an Option<String> type!

  assert_eq!(val, "the secret herb called orega-"); // value accessible!

  // What if you want to mutate the parser and remove KFC's secret recipe? Just use get_mut_map():
  let mut_map = config.get_mut_map();
  mut_map.get_mut("topsecret").unwrap().insert(String::from("KFC"), None);
  // And the secret is back in safety, remember that these are normal HashMap functions chained for convenience.

  // However very quickly see how that becomes cumbersome, so you can use the handy get() function from Ini
  let val = config.get("topsecret", "KFC"); // unwrapping will be an error because we just emptied it!

  assert_eq!(val, None); // as expected!
  Ok(())
}