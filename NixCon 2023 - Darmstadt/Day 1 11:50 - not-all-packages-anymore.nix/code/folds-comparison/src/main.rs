use std::io::Read;
use std::collections::HashMap;
use rnix::ast::HasEntry;
use rnix::ast::AttrSet;
use rnix::ast::Expr;
use rnix::Root;
use std::io;

fn main() {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents).unwrap();

    let root = match Root::parse(&contents).ok() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("Couldn't parse all-packages.nix file: {:?}", err);
            std::process::exit(1);
        }
    };

    let attribute_set = resulting_attrs(root.expr().unwrap());

    let mut attrs: Vec<String> = attribute_set.attrpath_values().into_iter().map(|d| {
        d.attrpath().unwrap().to_string()
    }).collect();

    attrs.sort();

    let mut shards = vec![];
    let mut entries: HashMap<String, Vec<String>> = HashMap::new();
    for attr in attrs {
        let shard: String = attr.to_lowercase().chars().take(2).collect();
        if let Some(value) = entries.get_mut(&shard) {
            value.push(attr);
        } else {
            shards.push(shard.clone());
            entries.insert(shard, vec![attr]);
        }
    }

    let mut counts = vec![];
    for shard in shards {
        let attrs = &entries[&shard];
        counts.push(attrs.len());
    }

    let linear = merge_linear(&counts);
    let tree = merge_tree(&counts);
    println!("{} {}", linear, tree);
}

fn merge_linear(counts: &[usize]) -> usize {
    if counts.is_empty() {
        return 0;
    }
    let first = counts[0];
    let rest = &counts[1..];
    merge_linear_go(first, rest)
}

fn merge_linear_go(first: usize, rest: &[usize]) -> usize {
    if rest.is_empty() {
        return first;
    }
    let second = rest[0];
    let rest = &rest[1..];

    first + second + merge_linear_go(first + second, rest)
}

fn merge_tree(counts: &[usize]) -> usize {
    if counts.is_empty() {
        return 0;
    }
    let (alloc, _) = merge_tree_go(counts);
    alloc
}

fn merge_tree_go(counts: &[usize]) -> (usize, usize) {
    if counts.len() == 1 {
        (counts[0], counts[0])
    } else {
        let mid = counts.len() / 2;
        let (leftalloc, leftreturned) = merge_tree_go(&counts[..mid]);
        let (rightalloc, rightreturned) = merge_tree_go(&counts[mid..]);
        (leftalloc + rightalloc + leftreturned + rightreturned, leftreturned + rightreturned)
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
