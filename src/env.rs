menv::require_envs! {
    (assert_env_vars, any_set, gen_help);

    tree_path, "INPUT_TREE_PATH", String,
    "INPUT_TREE_PATH should be set to the path to the tree json file you're using.";

    out_path, "OUTPUT_TREE_PATH", String,
    "OUTPUT_TREE_PATH should be set to the path to the nested tree json file you want to generate.";

    // flat_out_path, "OUTPUT_FLAT_PATH", String,
    // "OUTPUT_FLAT_PATH should be set to the path to the flat list file you want to generate";

    chosen_crate, "CRATE", String,
    "CRATE should be set to the crate you want to find all the dependencies of.";

    should_pretty?, "SHOULD_PRETTY", bool,
    "SHOULD_PRETTY can be set to true/false to decide whether the output json will be prettified.";

    should_count_dupes?, "SHOULD_COUNT_DUPES", bool,
    "SHOULD_COUNT_DUPES can be set to true/false to decide whether counts of duplicate rdeps are printed.";
}
