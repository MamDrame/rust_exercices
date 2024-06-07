/*Create a struct Film that has one field name of type String.

Create the following two functions:

Create a function take_film_name, it will return the name and consume the film (you should not be able to reuse it after you passed it to the function).
Create a function read_film_name, it will return the name without consuming the film (you can call the function multiple times with the same argument).
Expected functions

*/
pub struct Film {
    pub name: String,
}

pub fn read_film_name(film: &Film) -> &String {
    &film.name
}

pub fn take_film_name(film: Film) -> String {
    film.name
}