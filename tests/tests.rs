#[macro_use]
extern crate clap;

use std::collections::HashSet;

use clap::{App, Arg, ArgGroup};
use std::vec::Vec;

arg_enum!{
    #[derive(Debug)]
    enum Val1 {
        ValOne,
        ValTwo
    }
}
arg_enum!{
    #[derive(Debug)]
    pub enum Val2 {
        ValOne,
        ValTwo
    }
}
arg_enum!{
    enum Val3 {
        ValOne,
        ValTwo
    }
}
arg_enum!{
    pub enum Val4 {
        ValOne,
        ValTwo
    }
}

#[test]
#[cfg_attr(feature = "lints", allow(single_match))]
fn test_enums() {
    let v1_lower = "valone";
    let v1_camel = "ValOne";

    let v1_lp = v1_lower.parse::<Val1>().unwrap();
    let v1_cp = v1_camel.parse::<Val1>().unwrap();
    match v1_lp {
        Val1::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val1::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val2>().unwrap();
    let v1_cp = v1_camel.parse::<Val2>().unwrap();
    match v1_lp {
        Val2::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val2::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val3>().unwrap();
    let v1_cp = v1_camel.parse::<Val3>().unwrap();
    match v1_lp {
        Val3::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val3::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val4>().unwrap();
    let v1_cp = v1_camel.parse::<Val4>().unwrap();
    match v1_lp {
        Val4::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val4::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
}

#[test]
fn create_app() {
    let _ =
        App::new("test").version("1.0").author("kevin").about("does awesome things").get_matches();
}

#[test]
fn add_multiple_arg() {
    let _ = App::new("test")
                .args(&mut [
                    Arg::with_name("test").short("s"),
                    Arg::with_name("test2").short("l")])
                .get_matches();
}

#[test]
fn create_flag() {
    let _ = App::new("test")
                .arg(Arg::with_name("test")
                            .short("t")
                            .long("test")
                            .help("testing testing"))
                .get_matches();
}

#[test]
fn create_flag_usage() {
    let a = Arg::from_usage("[flag] -f 'some help info'");
    assert_eq!(a.name, "flag");
    assert_eq!(a.short.unwrap(), 'f');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("[flag] --flag 'some help info'");
    assert_eq!(b.name, "flag");
    assert_eq!(b.long.unwrap(), "flag");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--flag 'some help info'");
    assert_eq!(b.name, "flag");
    assert_eq!(b.long.unwrap(), "flag");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("[flag] -f --flag 'some help info'");
    assert_eq!(c.name, "flag");
    assert_eq!(c.short.unwrap(), 'f');
    assert_eq!(c.long.unwrap(), "flag");
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("[flag] -f... 'some help info'");
    assert_eq!(d.name, "flag");
    assert_eq!(d.short.unwrap(), 'f');
    assert!(d.long.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

    let e = Arg::from_usage("[flag] -f --flag... 'some help info'");
    assert_eq!(e.name, "flag");
    assert_eq!(e.long.unwrap(), "flag");
    assert_eq!(e.short.unwrap(), 'f');
    assert_eq!(e.help.unwrap(), "some help info");
    assert!(e.multiple);
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("-f --flag... 'some help info'");
    assert_eq!(e.name, "flag");
    assert_eq!(e.long.unwrap(), "flag");
    assert_eq!(e.short.unwrap(), 'f');
    assert_eq!(e.help.unwrap(), "some help info");
    assert!(e.multiple);
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("--flags");
    assert_eq!(e.name, "flags");
    assert_eq!(e.long.unwrap(), "flags");
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("--flags...");
    assert_eq!(e.name, "flags");
    assert_eq!(e.long.unwrap(), "flags");
    assert!(e.multiple);
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("[flags] -f");
    assert_eq!(e.name, "flags");
    assert_eq!(e.short.unwrap(), 'f');
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("[flags] -f...");
    assert_eq!(e.name, "flags");
    assert_eq!(e.short.unwrap(), 'f');
    assert!(e.multiple);
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());
}

#[test]
fn create_positional() {
    let _ = App::new("test")
                .arg(Arg::with_name("test")
                            .index(1)
                            .help("testing testing"))
                .get_matches();
}

#[test]
fn create_positional_usage() {
    let a = Arg::from_usage("[pos] 'some help info'");
    assert_eq!(a.name, "pos");
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("<pos> 'some help info'");
    assert_eq!(b.name, "pos");
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("[pos]... 'some help info'");
    assert_eq!(c.name, "pos");
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(!c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("<pos>... 'some help info'");
    assert_eq!(d.name, "pos");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

    let b = Arg::from_usage("<pos>");
    assert_eq!(b.name, "pos");
    assert!(!b.multiple);
    assert!(b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("[pos]...");
    assert_eq!(c.name, "pos");
    assert!(c.multiple);
    assert!(!c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());
}

#[test]
fn create_args_tabs_usage() {
    let a = Arg::from_usage("[pos]\t'some help info'");
    assert_eq!(a.name, "pos");
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("<pos>\t'some help info'");
    assert_eq!(b.name, "pos");
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("[pos]...\t'some help info'");
    assert_eq!(c.name, "pos");
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(!c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("<pos>...\t'some help info'");
    assert_eq!(d.name, "pos");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option() {
    let _ = App::new("test")
                .arg(Arg::with_name("test")
                            .short("t")
                            .long("test")
                            .takes_value(true)
                            .help("testing testing"))
                .get_matches();
}

#[test]
fn create_option_usage() {
	// Short only
    let a = Arg::from_usage("[option] -o [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.short.unwrap(), 'o');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o [opt] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert!(b.long.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> -o <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.short.unwrap(), 'o');
    assert!(c.long.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o <opt> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert!(d.long.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option] -o [opt]... 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.short.unwrap(), 'o');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let a = Arg::from_usage("[option]... -o [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.short.unwrap(), 'o');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o [opt]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert!(b.long.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> -o <opt>... 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.short.unwrap(), 'o');
    assert!(c.long.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let c = Arg::from_usage("<option>... -o <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.short.unwrap(), 'o');
    assert!(c.long.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o <opt>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert!(d.long.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

	// Long only

    let a = Arg::from_usage("[option] --opt [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--opt [option] 'some help info'");
    assert_eq!(b.name, "option");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("--opt <option> 'some help info'");
    assert_eq!(d.name, "option");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option] --opt [opt]... 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let a = Arg::from_usage("[option]... --opt [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--opt [option]... 'some help info'");
    assert_eq!(b.name, "option");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> --opt <opt>... 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let c = Arg::from_usage("<option>... --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("--opt <option>... 'some help info'");
    assert_eq!(d.name, "option");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

	// Long only with '='

    let a = Arg::from_usage("[option] --opt=[opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--opt=[option] 'some help info'");
    assert_eq!(b.name, "option");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("--opt=<option> 'some help info'");
    assert_eq!(d.name, "option");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option] --opt=[opt]... 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let a = Arg::from_usage("[option]... --opt=[opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--opt=[option]... 'some help info'");
    assert_eq!(b.name, "option");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> --opt=<opt>... 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let c = Arg::from_usage("<option>... --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("--opt=<option>... 'some help info'");
    assert_eq!(d.name, "option");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

	// Long and Short

    let a = Arg::from_usage("[option] -o --opt [option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o --opt [option] 'some help info'");
    assert_eq!(b.name, "option");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> -o --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o --opt <option> 'some help info'");
    assert_eq!(d.name, "option");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option]... -o --opt [option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o --opt [option]... 'some help info'");
    assert_eq!(b.name, "option");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option>... -o --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o --opt <option>... 'some help info'");
    assert_eq!(d.name, "option");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

	// Long and Short with '='

    let a = Arg::from_usage("[option] -o --opt=[option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o --opt=[option] 'some help info'");
    assert_eq!(b.name, "option");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> -o --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o --opt=<option> 'some help info'");
    assert_eq!(d.name, "option");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option]... -o --opt=[option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o --opt=[option]... 'some help info'");
    assert_eq!(b.name, "option");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option>... -o --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o --opt=<option>... 'some help info'");
    assert_eq!(d.name, "option");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_with_vals() {
    let d = Arg::from_usage("-o <opt> <opt> 'some help info'");
    assert_eq!(d.name, "opt");
    assert!(d.long.is_none());
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert_eq!(d.num_vals.unwrap(), 2);

    let d = Arg::from_usage("-o <opt> <opt>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert!(d.long.is_none());
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert_eq!(d.num_vals.unwrap(), 2);

    let d = Arg::from_usage("--opt <file> <mode>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert!(d.short.is_none());
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    let mut v = d.val_names.unwrap().into_iter().collect::<HashSet<_>>();
    for name in &["mode", "file"] {
        assert!(v.remove(name));
    }
    assert!(v.is_empty());
    assert_eq!(d.num_vals.unwrap(), 2);

    let d = Arg::from_usage("[myopt] --opt <file> <mode> 'some help info'");
    assert_eq!(d.name, "myopt");
    assert!(d.short.is_none());
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(!d.required);
    let mut v = d.val_names.unwrap().into_iter().collect::<HashSet<_>>();
    for name in &["mode", "file"] {
        assert!(v.remove(name));
    }
    assert!(v.is_empty());
    assert_eq!(d.num_vals.unwrap(), 2);

    let d = Arg::from_usage("--opt <option> <option> 'some help info'");
    assert_eq!(d.name, "option");
    assert!(d.short.is_none());
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(d.num_vals.unwrap(), 2);
}

#[test]
#[should_panic]
fn empty_group() {
    let _ = App::new("empty_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("vers")
            .required(true))
        .get_matches();
}

#[test]
#[should_panic]
fn empty_group_2() {
    let _ = App::new("empty_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("vers")
            .required(true)
            .args(&["ver", "major"]))
        .get_matches();
}

#[test]
#[should_panic]
fn errous_group() {
    let _ = App::new("errous_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("vers")
            .arg("vers")
            .required(true))
        .get_matches();
}
