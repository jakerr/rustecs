#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


use rustecs::{
	Components,
	EntityContainer,
};


world! {
	components Alpha, Beta;

	events Update;

	system update on(Update) with(Alpha, Beta);
}


pub type Alpha = bool;
pub type Beta  = bool;
