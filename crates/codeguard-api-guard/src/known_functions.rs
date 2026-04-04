use std::collections::HashMap;

/// Well-known library functions that are commonly called bare (without module prefix)
/// by LLMs. Maps function_name → (canonical_module, canonical_alias).
pub fn bare_call_map() -> HashMap<&'static str, (&'static str, &'static str)> {
    let mut m = HashMap::new();

    // pandas
    m.insert("read_csv", ("pandas", "pd"));
    m.insert("read_excel", ("pandas", "pd"));
    m.insert("read_json", ("pandas", "pd"));
    m.insert("read_sql", ("pandas", "pd"));
    m.insert("read_parquet", ("pandas", "pd"));
    m.insert("read_html", ("pandas", "pd"));
    m.insert("read_clipboard", ("pandas", "pd"));
    m.insert("read_table", ("pandas", "pd"));
    m.insert("DataFrame", ("pandas", "pd"));
    m.insert("Series", ("pandas", "pd"));
    m.insert("concat", ("pandas", "pd"));
    m.insert("merge", ("pandas", "pd"));
    m.insert("to_datetime", ("pandas", "pd"));
    m.insert("get_dummies", ("pandas", "pd"));

    // numpy
    m.insert("array", ("numpy", "np"));
    m.insert("zeros", ("numpy", "np"));
    m.insert("ones", ("numpy", "np"));
    m.insert("arange", ("numpy", "np"));
    m.insert("linspace", ("numpy", "np"));
    m.insert("reshape", ("numpy", "np"));
    m.insert("concatenate", ("numpy", "np"));
    m.insert("mean", ("numpy", "np"));
    m.insert("std", ("numpy", "np"));
    m.insert("dot", ("numpy", "np"));
    m.insert("matmul", ("numpy", "np"));
    m.insert("random.seed", ("numpy", "np"));

    // requests
    m.insert("get", ("requests", "requests"));
    m.insert("post", ("requests", "requests"));
    m.insert("put", ("requests", "requests"));
    m.insert("delete", ("requests", "requests"));
    m.insert("patch", ("requests", "requests"));
    m.insert("head", ("requests", "requests"));
    m.insert("Session", ("requests", "requests"));

    // json (stdlib but commonly missed)
    m.insert("loads", ("json", "json"));
    m.insert("dumps", ("json", "json"));
    m.insert("load", ("json", "json"));
    m.insert("dump", ("json", "json"));

    // os.path
    m.insert("join", ("os.path", "os.path"));
    m.insert("exists", ("os.path", "os.path"));
    m.insert("isfile", ("os.path", "os.path"));
    m.insert("isdir", ("os.path", "os.path"));
    m.insert("basename", ("os.path", "os.path"));
    m.insert("dirname", ("os.path", "os.path"));

    // matplotlib
    m.insert("plot", ("matplotlib.pyplot", "plt"));
    m.insert("show", ("matplotlib.pyplot", "plt"));
    m.insert("savefig", ("matplotlib.pyplot", "plt"));
    m.insert("xlabel", ("matplotlib.pyplot", "plt"));
    m.insert("ylabel", ("matplotlib.pyplot", "plt"));
    m.insert("title", ("matplotlib.pyplot", "plt"));
    m.insert("legend", ("matplotlib.pyplot", "plt"));
    m.insert("figure", ("matplotlib.pyplot", "plt"));
    m.insert("subplot", ("matplotlib.pyplot", "plt"));
    m.insert("imshow", ("matplotlib.pyplot", "plt"));

    // sklearn
    m.insert(
        "train_test_split",
        ("sklearn.model_selection", "sklearn.model_selection"),
    );
    m.insert("accuracy_score", ("sklearn.metrics", "sklearn.metrics"));
    m.insert("confusion_matrix", ("sklearn.metrics", "sklearn.metrics"));
    m.insert(
        "classification_report",
        ("sklearn.metrics", "sklearn.metrics"),
    );

    // torch
    m.insert("tensor", ("torch", "torch"));
    m.insert("no_grad", ("torch", "torch"));

    m
}
