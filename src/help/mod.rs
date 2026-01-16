use crate::ArgumentList;

mod help_impl;

pub(crate) fn print_help(arguments: ArgumentList) {
    help_impl::print_help(arguments);
}