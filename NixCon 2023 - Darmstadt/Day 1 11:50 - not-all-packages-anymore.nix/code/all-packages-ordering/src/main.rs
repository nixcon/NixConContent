use rnix::ast::HasEntry;
use rnix::ast::AttrSet;
use rnix::ast::Expr;
use rnix::Root;
use std::fs::read_to_string;
use std::env;
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = read_to_string(&path).unwrap();

    let root = match Root::parse(&contents).ok() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("Couldn't parse all-packages.nix file {:?}: {:?}", path, err);
            std::process::exit(1);
        }
    };

    let attribute_set = resulting_attrs(root.expr().unwrap());

    let mut attrs: Vec<(usize, String)> = attribute_set.attrpath_values().into_iter().map(|d| {
        d.attrpath().unwrap().to_string()
    }).enumerate().collect();

    attrs.sort_by_key(|(_, s)| s.clone());
    
    let l = attrs.len() as f32;

    let mut full: Vec<(usize, f32, usize, f32, String)> = attrs.into_iter().enumerate().map(|(expected, (actual, attr))| {
        (expected, expected as f32 / l, actual, actual as f32 / l, attr)
    }).collect();

    full.sort_by_key(|(_, _, actual, _, _)| actual.clone());

    for (expected, expected_f, actual, actual_f, attr) in full.into_iter() {
        let value = serde_json::json!({
            "expected": expected,
            "expected_f": expected_f,
            "actual": actual,
            "actual_f": actual_f,
            "attribute": attr,
        });
        let str = serde_json::to_string(&value).unwrap();
        println!("{str}");
    }
}

fn resulting_attrs(expr: Expr) -> AttrSet {
    //let pref : String = String::from(expr.to_string()).chars().take(100).collect();
    //eprintln!("{}\n#########", &pref);
    match expr {
        Expr::Lambda(it) => resulting_attrs(it.body().unwrap()),
        Expr::With(it) => resulting_attrs(it.body().unwrap()),
        Expr::AttrSet(it) => it,
        Expr::LetIn(it) => {
            if let Some(attr) = it.attrpath_values().find(|x| x.attrpath().unwrap().to_string() == "pkgsFun") {
                return resulting_attrs(attr.value().unwrap());
            }
            if let Some(attr) = it.attrpath_values().find(|x| x.attrpath().unwrap().to_string() == "self_") {
                return resulting_attrs(attr.value().unwrap());
            }
            if let Some(attr) = it.attrpath_values().find(|x| x.attrpath().unwrap().to_string() == "pkgs") {
                return resulting_attrs(attr.value().unwrap());
            }
            if let Some(attr) = it.attrpath_values().find(|x| x.attrpath().unwrap().to_string() == "pkgs_") {
                return resulting_attrs(attr.value().unwrap());
            }
            resulting_attrs(it.body().unwrap())
        },
        Expr::BinOp(it) => {
            resulting_attrs(it.rhs().unwrap())
        },
        e => {
            panic!("{:?}", &e.to_string());
        },
    }
}
