use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

mod env;

#[derive(serde::Deserialize)]
struct DepInfo {
    #[serde(flatten)]
    crates: HashMap<String, Vec<String>>,
}

#[derive(serde::Serialize)]
struct NestedTree {
    #[serde(flatten)]
    rdeps: HashMap<String, NestedTree>,
}

fn main() {
    if env::any_set() {
        env::assert_env_vars();
    } else {
        println!("# Environment Variables Help\n{}", env::gen_help());
        return
    }

    let info: DepInfo = serde_json::from_str(&std::fs::read_to_string(env::tree_path()).unwrap()).unwrap();
    let mut visited = HashSet::new();
    let nested = transform_tree(&info, &mut visited, &env::chosen_crate());
    std::fs::write(env::out_path(), if env::should_pretty().unwrap_or(false) {
        serde_json::to_string_pretty(&nested).unwrap()
    } else {
        serde_json::to_string(&nested).unwrap()
    }).unwrap();

    if env::should_count_dupes().unwrap_or(false) {
        println!("Dupes in same level: {:?}", &DUP_IN_SAME_LEVEL_COUNT);
        println!("Dupes not in the same level: {:?}", &NOT_DUP_IN_SAME_LEVEL_COUNT);
    }
}

static DUP_IN_SAME_LEVEL_COUNT: AtomicU64 = AtomicU64::new(0);
static NOT_DUP_IN_SAME_LEVEL_COUNT: AtomicU64 = AtomicU64::new(0);

fn transform_tree(dep_info: &DepInfo, visited: &mut HashSet<String>, target: &str) -> NestedTree {
    let mut rdep_tree = HashMap::new();
    for name in &dep_info.crates[target] {
        if visited.insert(name.to_string()) {
            let inner_tree = transform_tree(dep_info, visited, name);
            assert!(rdep_tree.insert(name.to_string(), inner_tree).is_none());
        } else {
            // let res = rdep_tree.insert(name.to_string(), NestedTree { rdeps: HashMap::new() });
            if rdep_tree.contains_key(name) {
                DUP_IN_SAME_LEVEL_COUNT.fetch_add(1, Ordering::Relaxed);
                // println!("Parent: {target}, Child: {name}");
            } else {
                rdep_tree.insert(name.to_string(), NestedTree { rdeps: HashMap::new() });
                NOT_DUP_IN_SAME_LEVEL_COUNT.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
    NestedTree { rdeps: rdep_tree }
}
