use std::collections::HashMap;

use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol::Symbol;

pub struct SymbolTables {
    class_table: HashMap<String, Symbol>,
    subroutine_table: HashMap<String, Symbol>,
    pub class_name: String,
}

impl SymbolTables {
    pub fn new() -> Self {
        SymbolTables {
            class_table: Default::default(),
            subroutine_table: Default::default(),
            class_name: "".to_string(),
        }
    }

    pub fn start_subroutine(&mut self) {
        self.subroutine_table = Default::default()
    }

    pub fn define(&mut self, name: &str, type_name: &str, kind: &Kind) {
        match kind {
            Kind::Static | Kind::Field => {
                let index = self.var_count(Kind::from(kind));
                self.class_table
                    .insert(String::from(name), Symbol::new(type_name, kind, index));
            }
            Kind::Argument | Kind::Var => {
                let index = self.var_count(Kind::from(kind));
                self.subroutine_table
                    .insert(String::from(name), Symbol::new(type_name, kind, index));
            }
        }
    }

    pub fn var_count(&mut self, kind: Kind) -> usize {
        match kind {
            Kind::Static | Kind::Field => {
                self.class_table.values().filter(|s| s.kind == kind).count()
            }
            Kind::Argument | Kind::Var => self
                .subroutine_table
                .values()
                .filter(|s| s.kind == kind)
                .count(),
        }
    }

    pub fn get(&mut self, name: &str) -> Option<&Symbol> {
        match self.subroutine_table.get(name) {
            Some(s) => Some(s),
            None => match self.class_table.get(name) {
                Some(s) => Some(s),
                None => None,
            },
        }
    }

    #[allow(dead_code)]
    pub fn kind_of(&mut self, name: &str) -> Option<&Kind> {
        match self.subroutine_table.get(name) {
            Some(s) => Some(&s.kind),
            None => match self.class_table.get(name) {
                Some(s) => Some(&s.kind),
                None => None,
            },
        }
    }

    #[allow(dead_code)]
    pub fn type_of(&mut self, name: &str) -> Option<String> {
        match self.subroutine_table.get(name) {
            Some(s) => Some(String::from(&s.type_name)),
            None => self
                .class_table
                .get(name)
                .map(|s| String::from(&s.type_name)),
        }
    }

    #[allow(dead_code)]
    pub fn index_of(&mut self, name: &str) -> Option<usize> {
        match self.subroutine_table.get(name) {
            Some(s) => Some(s.index),
            None => self.class_table.get(name).map(|s| s.index),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol::Symbol;
    use crate::symbol_table::symbol_tables::SymbolTables;

    #[test]
    fn can_start_subroutine() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Var);
        symbol_tables.start_subroutine();
        let actual = symbol_tables.subroutine_table.len();
        assert_eq!(0, actual);
    }

    #[test]
    fn can_define_class_scope_symbol() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Static);
        let actual = symbol_tables.class_table.get("is_test").unwrap();
        assert_eq!(Symbol::new("boolean", &Kind::Static, 0), *actual);
    }

    #[test]
    fn can_define_subroutine_scope_symbol() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Var);
        let actual = symbol_tables.subroutine_table.get("is_test").unwrap();
        assert_eq!(Symbol::new("boolean", &Kind::Var, 0), *actual);
    }

    #[test]
    fn can_count_number_of_kind() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("count", "int", &Kind::Var);
        symbol_tables.define("is_test", "boolean", &Kind::Var);
        let actual = symbol_tables.var_count(Kind::Var);
        assert_eq!(2, actual);
    }

    #[test]
    fn can_get_kind_of_subroutine_scope_from_name() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Argument);
        let actual = symbol_tables.kind_of("is_test").unwrap();
        assert_eq!(Kind::Argument, *actual);
    }

    #[test]
    fn can_get_kind_of_class_scope_from_name() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Field);
        let actual = symbol_tables.kind_of("is_test").unwrap();
        assert_eq!(Kind::Field, *actual)
    }

    #[test]
    fn can_get_kind_of_none() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Field);
        let actual = symbol_tables.kind_of("hoge");
        assert!(actual.is_none());
    }

    #[test]
    fn can_get_type_of_subroutine_scope_from_name() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Argument);
        let actual = symbol_tables.type_of("is_test").unwrap();
        assert_eq!("boolean", actual);
    }

    #[test]
    fn can_get_type_of_class_scope_from_name() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Field);
        let actual = symbol_tables.type_of("is_test").unwrap();
        assert_eq!("boolean", actual)
    }

    #[test]
    fn can_get_index_of_subroutine_scope_from_name() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Argument);
        let actual = symbol_tables.index_of("is_test").unwrap();
        assert_eq!(0, actual);
    }

    #[test]
    fn can_get_index_of_class_scope_from_name() {
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("is_test", "boolean", &Kind::Field);
        let actual = symbol_tables.index_of("is_test").unwrap();
        assert_eq!(0, actual)
    }
}
