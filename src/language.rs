use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    JavaScript,
    Python,
    C,
    Cpp,
}

impl Language {
    /// Get the file extension for this language
    pub fn extension(&self) -> &'static str {
        match self {
            Language::JavaScript => "js",
            Language::Python => "py",
            Language::C => "c",
            Language::Cpp => "cpp",
        }
    }

    /// Get the display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::JavaScript => "JavaScript",
            Language::Python => "Python",
            Language::C => "C",
            Language::Cpp => "C++",
        }
    }

    /// Get short name for compact display
    pub fn short_name(&self) -> &'static str {
        match self {
            Language::JavaScript => "JS",
            Language::Python => "Py",
            Language::C => "C",
            Language::Cpp => "C++",
        }
    }

    /// Cycle to the next language
    pub fn next(&self) -> Language {
        match self {
            Language::JavaScript => Language::Python,
            Language::Python => Language::C,
            Language::C => Language::Cpp,
            Language::Cpp => Language::JavaScript,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Parse JavaScript function parameters from a signature
/// "var twoSum = function(nums, target)" -> ["nums", "target"]
pub fn parse_js_params(js_sig: &str) -> Vec<String> {
    if let Some(start) = js_sig.find('(') {
        if let Some(end) = js_sig.rfind(')') {
            let params_str = &js_sig[start + 1..end];
            return params_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
    Vec::new()
}

/// Derive a function signature for a target language from JavaScript
pub fn derive_signature(func_name: &str, js_sig: &str, lang: Language) -> String {
    let params = parse_js_params(js_sig);

    match lang {
        Language::JavaScript => js_sig.to_string(),
        Language::Python => {
            if params.is_empty() {
                format!("def {}():", func_name)
            } else {
                format!("def {}({}):", func_name, params.join(", "))
            }
        }
        Language::C => {
            // For C, infer types from function/param names
            let func_lower = func_name.to_lowercase();

            // Infer return type from function name
            let return_type = if func_lower.starts_with("is") || func_lower.starts_with("has")
                || func_lower.starts_with("can") || func_lower.starts_with("contains")
                || func_lower.starts_with("valid")
            {
                "bool"
            } else if func_lower.starts_with("count") || func_lower.starts_with("max")
                || func_lower.starts_with("min") || func_lower.starts_with("length")
                || func_lower.starts_with("find") || func_lower == "reverse"
                || func_lower.contains("missing") || func_lower == "getsum"
            {
                "int"
            } else {
                "int*" // Default to array return with returnSize
            };

            if params.is_empty() {
                format!("{} {}(void)", return_type, func_name)
            } else {
                let mut c_params: Vec<String> = Vec::new();
                for p in &params {
                    let p_lower = p.to_lowercase();
                    // Arrays need pointer + size
                    if p_lower.contains("num") || p_lower.contains("arr") || p_lower == "prices"
                        || p_lower == "heights" || p_lower == "intervals" || p_lower == "strs"
                    {
                        c_params.push(format!("int* {}", p));
                        c_params.push(format!("int {}Size", p));
                    } else if p_lower == "s" || p_lower == "t" || p_lower == "word" {
                        c_params.push(format!("char* {}", p));
                    } else {
                        c_params.push(format!("int {}", p));
                    }
                }
                // Add returnSize for array returns
                if return_type == "int*" {
                    c_params.push("int* returnSize".to_string());
                }
                format!("{} {}({})", return_type, func_name, c_params.join(", "))
            }
        }
        Language::Cpp => {
            // For C++, infer types from function/param names (user may need to adjust)
            let func_lower = func_name.to_lowercase();

            // Infer return type from function name
            let return_type = if func_lower.starts_with("is") || func_lower.starts_with("has")
                || func_lower.starts_with("can") || func_lower.starts_with("contains")
                || func_lower.starts_with("valid")
            {
                "bool"
            } else if func_lower.starts_with("count") && !func_lower.contains("bits")
                || func_lower.starts_with("max") || func_lower.starts_with("min")
                || func_lower.starts_with("length") || func_lower.starts_with("find")
                || func_lower == "reverse" || func_lower.contains("missing")
                || func_lower == "getsum" || func_lower.contains("reversebits")
            {
                "int"
            } else if func_lower.starts_with("two") || func_lower.contains("top")
                || func_lower.ends_with("bits") || func_lower.ends_with("sum")
            {
                "vector<int>"
            } else if func_lower.starts_with("group") {
                "vector<vector<string>>"
            } else {
                "auto"  // Fallback - return type deduction
            };

            if params.is_empty() {
                format!("{} {}()", return_type, func_name)
            } else {
                let cpp_params: Vec<String> = params
                    .iter()
                    .map(|p| {
                        // Heuristic: 'nums', 'arr', 'list' -> vector<int>&
                        // 'strs', 'words' -> vector<string>&
                        // 's', 't' -> string
                        // 'n', 'k', 'target' -> int
                        let p_lower = p.to_lowercase();
                        if p_lower.contains("num") || p_lower.contains("arr") || p_lower == "prices" || p_lower == "heights" || p_lower == "intervals" {
                            format!("vector<int>& {}", p)
                        } else if p_lower == "strs" || p_lower == "words" {
                            format!("vector<string>& {}", p)
                        } else if p_lower == "s" || p_lower == "t" || p_lower == "word" {
                            format!("string {}", p)
                        } else {
                            format!("int {}", p)
                        }
                    })
                    .collect();
                format!("{} {}({})", return_type, func_name, cpp_params.join(", "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_js_params() {
        assert_eq!(
            parse_js_params("var twoSum = function(nums, target)"),
            vec!["nums", "target"]
        );
        assert_eq!(
            parse_js_params("var isValid = function(s)"),
            vec!["s"]
        );
        assert_eq!(
            parse_js_params("var maxProfit = function()"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_derive_signature() {
        let js_sig = "var twoSum = function(nums, target)";

        assert_eq!(
            derive_signature("twoSum", js_sig, Language::Python),
            "def twoSum(nums, target):"
        );
        // C signature includes array size parameter
        assert!(
            derive_signature("twoSum", js_sig, Language::C).contains("numsSize")
        );
        // C++ infers vector type for nums
        assert!(
            derive_signature("twoSum", js_sig, Language::Cpp).contains("vector<int>")
        );
    }

    #[test]
    fn test_language_cycle() {
        assert_eq!(Language::JavaScript.next(), Language::Python);
        assert_eq!(Language::Cpp.next(), Language::JavaScript);
    }
}
