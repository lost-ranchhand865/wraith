use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

const INTROSPECT_SCRIPT: &str = r##"
import sys, json, importlib, inspect, warnings

def introspect(module_name, attr_name):
    result = {
        "exists": False,
        "module_found": False,
        "kind": None,
        "signature": None,
        "deprecated": False,
        "all_attributes": [],
        "closest_match": None,
    }

    try:
        mod = importlib.import_module(module_name)
    except (ImportError, ModuleNotFoundError):
        return result

    result["module_found"] = True

    result["all_attributes"] = [a for a in dir(mod) if not a.startswith("_")]

    if not hasattr(mod, attr_name):
        # Find closest match
        from difflib import get_close_matches
        matches = get_close_matches(attr_name, result["all_attributes"], n=1, cutoff=0.6)
        if matches:
            result["closest_match"] = matches[0]
        return result

    result["exists"] = True
    obj = getattr(mod, attr_name)

    if callable(obj):
        result["kind"] = "function"
        try:
            sig = inspect.signature(obj)
            params = []
            has_var_keyword = False
            for name, param in sig.parameters.items():
                kind = str(param.kind)
                if "VAR_KEYWORD" in kind:
                    has_var_keyword = True
                params.append({
                    "name": name,
                    "kind": kind,
                    "has_default": param.default is not inspect.Parameter.empty,
                })
            result["signature"] = {
                "params": params,
                "has_var_keyword": has_var_keyword,
            }
        except (ValueError, TypeError):
            pass

        # Check deprecation — 4 tiers (PEP 702 > source analysis > descriptor > docstring)
        # Tier 1: PEP 702 __deprecated__ attribute (Python 3.13+, warnings module)
        if hasattr(obj, "__deprecated__"):
            result["deprecated"] = True
        # Tier 2: Inspect source for unconditional deprecation warning
        if not result["deprecated"]:
            try:
                src_lines = inspect.getsource(obj).split("\n")
                # Find warnings.warn or warnings._deprecated at function body level
                # (indented exactly one level from def). Conditional deprecation
                # (inside if/try) means only specific usage is deprecated, not the function.
                body_indent = None
                for line in src_lines:
                    stripped = line.lstrip()
                    if stripped and not stripped.startswith("def ") and not stripped.startswith("@") and not stripped.startswith("#"):
                        body_indent = len(line) - len(stripped)
                        break
                if body_indent is not None:
                    for line in src_lines:
                        stripped = line.lstrip()
                        indent = len(line) - len(stripped)
                        if indent == body_indent and ("warnings._deprecated" in stripped or "warnings.warn" in stripped):
                            result["deprecated"] = True
                            break
            except (OSError, TypeError):
                pass
        # Tier 3: Check if accessing triggers descriptor-based warnings
        if not result["deprecated"]:
            with warnings.catch_warnings(record=True) as w:
                warnings.simplefilter("always", DeprecationWarning)
                try:
                    _ = getattr(mod, attr_name)
                    if any(issubclass(x.category, DeprecationWarning) for x in w):
                        result["deprecated"] = True
                except Exception:
                    pass
        # Tier 4: Docstring — only first line starting with "deprecated"
        if not result["deprecated"]:
            try:
                doc = inspect.getdoc(obj) or ""
                first_line = doc.strip().split("\n")[0].lower().strip()
                if first_line.startswith("deprecated") or ".. deprecated" in first_line:
                    result["deprecated"] = True
            except Exception:
                pass
    else:
        result["kind"] = "attribute"

    return result

data = json.loads(sys.stdin.read())
results = {}
for q in data.get("queries", []):
    key = f"{q['module']}.{q['attribute']}"
    results[key] = introspect(q["module"], q["attribute"])

json.dump(results, sys.stdout)
"##;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectResult {
    pub exists: bool,
    #[serde(default)]
    pub module_found: bool,
    pub kind: Option<String>,
    pub signature: Option<SignatureInfo>,
    pub deprecated: bool,
    pub all_attributes: Vec<String>,
    pub closest_match: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInfo {
    pub params: Vec<ParamInfo>,
    pub has_var_keyword: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    pub name: String,
    pub kind: String,
    pub has_default: bool,
}

pub struct PythonIntrospector {
    python_exec: String,
}

impl PythonIntrospector {
    pub fn new(python_exec: String) -> Self {
        Self { python_exec }
    }

    pub fn batch_introspect(
        &self,
        queries: &[(String, String)],
    ) -> Result<HashMap<String, IntrospectResult>> {
        if queries.is_empty() {
            return Ok(HashMap::new());
        }

        let input = serde_json::json!({
            "queries": queries.iter().map(|(m, a)| {
                serde_json::json!({"module": m, "attribute": a})
            }).collect::<Vec<_>>()
        });

        let mut child = Command::new(&self.python_exec)
            .args(["-c", INTROSPECT_SCRIPT])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        {
            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(input.to_string().as_bytes())?;
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Python introspection failed: {stderr}");
        }

        let stdout = String::from_utf8(output.stdout)?;
        let results: HashMap<String, IntrospectResult> = serde_json::from_str(&stdout)?;
        Ok(results)
    }
}
