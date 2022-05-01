#[macro_use]
extern crate sql_reveser_error;

pub mod common;
pub mod gen_struct;
pub mod mysql_struct;
pub mod postgres_struct;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
