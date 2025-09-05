mod print_sth; 

use crate::print_sth::pub_pri;

pub fn do_print_job() {
    pub_pri::public_function();
}