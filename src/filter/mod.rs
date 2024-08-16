mod filter;
use filter::Filter;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FILTER: Filter = Filter::new();
}