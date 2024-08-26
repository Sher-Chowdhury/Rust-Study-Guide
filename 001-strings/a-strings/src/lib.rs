pub fn get_alias(super_hero: &str) -> String {

    if super_hero == "Superman" {
        return String::from("Cark") ;
    } else if super_hero == "Batman" { 
        return String::from("Bruce")
    } else {
        return String::from("unkown")        
    }

}